use crate::{
    http_route::HttpMethod,
    response::Response
};
use std::convert::Into;

#[derive(Debug)]
struct HttpHandler(crate::HttpRoute, i32);

#[derive(Debug)]
pub struct HttpServer {
    routes: Vec<HttpHandler>
}

impl HttpServer {
    pub fn new() -> HttpServer {
        HttpServer { routes: Vec::new() }
    }

    pub fn get<Responsable>(
        &mut self,
        path: &'static str,
        _handler: fn() -> Responsable,
    ) -> &mut Self
    where
        Responsable: Into<Response>
    {
        self.routes.push(HttpHandler(crate::HttpRoute {
            method: HttpMethod::GET,
            route: path.to_string()
        }, 7));

        self
    }

    pub fn post<Responsable>(
        &mut self,
        path: &'static str,
        _handler: fn() -> Responsable,
    ) -> &mut Self
    where
        Responsable: Into<Response>
    {
        self.routes.push(HttpHandler(crate::HttpRoute {
            method: HttpMethod::POST,
            route: path.to_string()
        }, 7));

        self
    }
}
