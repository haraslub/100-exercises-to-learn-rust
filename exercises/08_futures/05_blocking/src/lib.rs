// TODO: the `echo` server uses non-async primitives.
//  When running the tests, you should observe that it hangs, due to a
//  deadlock between the caller and the server.
//  Use `spawn_blocking` inside `echo` to resolve the issue.
use std::io::{Read, Write};
use tokio::net::TcpListener;

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    // Infinite loop to continuously accept new connections
    loop {
        // Step 1: Accept a new connection (async, non-blocking)
        // This yields control back to the runtime if no connection is ready
        let (socket, _) = listener.accept().await?;
        
        // Step 2: Convert the async Tokio socket to a standard library socket
        // This is necessary because we'll use synchronous I/O operations
        let mut socket = socket.into_std()?;
        
        // Step 3: Set the socket to blocking mode
        // This ensures that read/write operations will block until data is available
        // (required for synchronous I/O operations)
        socket.set_nonblocking(false)?;
        
        // Step 4: Spawn the blocking I/O operations on a separate thread pool
        // This prevents blocking the async runtime thread, allowing the loop to
        // continue accepting new connections while this connection is being processed
        // The `move` keyword transfers ownership of `socket` into the closure
        let _ = tokio::task::spawn_blocking(move || -> Result<(), anyhow::Error> {
            // Step 4a: Create a buffer to store incoming data
            let mut buffer = Vec::new();
            
            // Step 4b: Read all data from the socket (blocking operation)
            // This will block the thread pool thread, not the async runtime thread
            socket.read_to_end(&mut buffer)?;
            
            // Step 4c: Write all the data back to the socket (blocking operation)
            // Echo the received data back to the client
            socket.write_all(&buffer)?;
            
            Ok(())
        });
        // Note: We don't await the handle, so the loop continues immediately
        // to accept the next connection while this one is being processed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (listener, addr) = bind_random().await;
        tokio::spawn(echo(listener));

        let requests = vec![
            "hello here we go with a long message",
            "world",
            "foo",
            "bar",
        ];
        let mut join_set = JoinSet::new();

        for request in requests {
            join_set.spawn(async move {
                let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                let (mut reader, mut writer) = socket.split();

                // Send the request
                writer.write_all(request.as_bytes()).await.unwrap();
                // Close the write side of the socket
                writer.shutdown().await.unwrap();

                // Read the response
                let mut buf = Vec::with_capacity(request.len());
                reader.read_to_end(&mut buf).await.unwrap();
                assert_eq!(&buf, request.as_bytes());
            });
        }

        // Wait for all spawned client tasks to complete
        // `join_next()` returns `Some(Result)` while there are still tasks running,
        // and `None` when all tasks have finished
        while let Some(outcome) = join_set.join_next().await {
            // Check if the task completed with an error (panic or cancellation)
            if let Err(e) = outcome {
                // Try to extract the panic payload if the task panicked
                // `try_into_panic()` returns `Ok(Box<dyn Any + Send>)` if it was a panic,
                // or `Err(JoinError)` if it was a cancellation
                if let Ok(reason) = e.try_into_panic() {
                    // Re-panic in the current thread so the test fails properly
                    // This is necessary because panics in spawned tasks don't automatically
                    // fail the test - we need to propagate them explicitly
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
