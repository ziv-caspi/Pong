use anyhow::{anyhow, Result};
use crossbeam::channel::{unbounded, Receiver, Sender};
use std::{thread, time::Duration};

#[derive(Clone)]
pub struct RpcMessage<TRequest, TResponse> {
    pub message: TRequest,
    pub response_channel: Sender<TResponse>,
}

impl<TRequest, TResponse> RpcMessage<TRequest, TResponse> {
    pub fn new(message: TRequest) -> (RpcMessage<TRequest, TResponse>, Receiver<TResponse>) {
        let (s, r) = unbounded();
        (
            RpcMessage {
                message: message,
                response_channel: s,
            },
            r,
        )
    }
}

pub struct Channel<T> {
    pub reciever: Receiver<T>,
    pub sender: Sender<T>,
}

#[derive(Clone)]
pub struct RpcMethod<TRequest, TResponse> {
    pub sender: Sender<RpcMessage<TRequest, TResponse>>,
}

impl<TRequest, TResponse> RpcMethod<TRequest, TResponse> {
    pub fn invoke(&self, request: TRequest, timeout: Duration) -> Result<TResponse> {
        let (response_sender, response_reciever) = unbounded::<TResponse>();
        self.sender
            .send(RpcMessage {
                message: request,
                response_channel: response_sender,
            })
            .or(Err(anyhow!("could not send request")))?;

        let response = response_reciever.recv_timeout(timeout)?;
        Ok(response)
    }
}

pub struct GenericInnerRpcHandler<Method, Inner>
where
    Method: 'static + Send + Sync,
    Inner: 'static + Send + Sync,
{
    inner: Inner,
    handler: Box<dyn Fn(&mut Inner, Method) -> () + Send + Sync>,
    pub general_channel: Channel<Method>,
}

impl<Method, Inner> GenericInnerRpcHandler<Method, Inner>
where
    Method: Send + Sync,
    Inner: Send + Sync,
{
    pub fn new(
        innder: Inner,
        handler: Box<dyn Fn(&mut Inner, Method) -> () + Send + Sync>,
    ) -> Self {
        let (g_s, g_r) = unbounded();
        GenericInnerRpcHandler {
            inner: innder,
            general_channel: Channel {
                reciever: g_r,
                sender: g_s,
            },
            handler,
        }
    }

    pub fn start(self) {
        // here you could do threadpool stuff if you wanted to
        thread::spawn(|| self.handle_requests());
    }

    fn handle_requests(mut self) -> ! {
        let handler = &self.handler;
        loop {
            let request = self.general_channel.reciever.recv().unwrap();
            handler.as_ref()(&mut self.inner, request);
        }
    }
}

#[derive(Clone)]
pub struct ThreadSafeAPI<API> {
    channel: Sender<API>,
}

impl<API> ThreadSafeAPI<API> {
    pub fn new(channel: Sender<API>) -> Self {
        Self { channel }
    }

    pub fn call<TResponse>(
        &self,
        message: API,
        response_channel: Receiver<TResponse>,
    ) -> Result<TResponse> {
        if let Err(e) = self.channel.send(message) {
            return Err(anyhow!(e.to_string()));
        }

        let response = response_channel
            .recv_timeout(Duration::from_secs(3))
            .or(Err(anyhow!("could not read response")))?;

        Ok(response)
    }
}
