use std::collections::HashMap;
use std::sync::Arc;

use suika::{
    middleware::{
        CorsMiddleware, FaviconMiddleware, LoggerMiddleware, StaticFileMiddleware,
        WasmFileMiddleware,
    },
    server::{Router, Server},
    templates::{TemplateEngine, TemplateValue},
};

fn main() {
    let mut server = Server::new("127.0.0.1:8080");
    let mut main_router = Router::new("/");

    let template_engine = {
        let mut engine = TemplateEngine::new();

        engine
            .load_templates_from_directory("crates/suika_example/templates")
            .expect("Failed to load templates from directory");

        engine
    };

    server.use_templates(template_engine);

    main_router.add_route(Some("GET"), r"/$", |_req, res| {
        Box::pin(async move {
            if let Err(e) = res.send_file("crates/suika_example/index.html").await {
                res.error(e).await;
            }
            Ok(())
        })
    });

    main_router.add_route(Some("GET"), "/hello",  |_req, res| {
        Box::pin(async move {
            let mut context = HashMap::new();

            context.insert(
                "name".to_string(),
                TemplateValue::String("World".to_string()),
            );

            res.set_status(200).await;
            res.render_template("hello.html", &context).await?;

            Ok(())
        })
    });

    main_router.add_route(Some("GET"), "/include", |_req, res| {
        Box::pin(async move {
            let mut context = HashMap::new();

            context.insert(
                "name".to_string(),
                TemplateValue::String("World".to_string()),
            );

            res.set_status(200).await;
            res.render_template("include.html", &context).await?;

            Ok(())
        })
    });

    main_router.add_route(Some("GET"), "/conditional", |_req, res| {
        Box::pin(async move {
            let mut context = HashMap::new();

            context.insert("is_member".to_string(), TemplateValue::Boolean(true));
            context.insert("name".to_string(), TemplateValue::String("Bob".to_string()));

            res.set_status(200).await;
            res.render_template("conditional.html", &context).await?;

            Ok(())
        })
    });

    main_router.add_route(Some("GET"), "/loop", |_req, res| {
        Box::pin(async move {
            let mut context = HashMap::new();

            context.insert(
                "items".to_string(),
                TemplateValue::Array(vec![
                    TemplateValue::String("One".to_string()),
                    TemplateValue::String("Two".to_string()),
                    TemplateValue::String("Three".to_string()),
                ]),
            );

            res.set_status(200).await;
            res.render_template("loop.html", &context).await?;

            Ok(())
        })
    });

    main_router.add_route(Some("GET"), "/user", |_req, res| {
        Box::pin(async move {
            let mut user = HashMap::new();

            user.insert(
                "name".to_string(),
                TemplateValue::String("Alice".to_string()),
            );
            user.insert("age".to_string(), TemplateValue::String("30".to_string()));
            user.insert(
                "email".to_string(),
                TemplateValue::String("alice@example.com".to_string()),
            );

            let mut context = HashMap::new();
            context.insert("user".to_string(), TemplateValue::Object(user));

            res.set_status(200).await;
            res.render_template("user.html", &context).await?;

            Ok(())
        })
    });

    main_router.add_route(Some("GET"), r"/items/(?P<id>\d+)$", |req, res| {
        Box::pin(async move {
            res.set_status(200).await;
            let item_id = req.param("id").map(|s| s.to_string()).unwrap_or_default();
            res.body(format!("You requested item with ID: {}", item_id))
                .await;
            Ok(())
        })
    });

    let mut user_router = Router::new("/users");

    user_router.add_route(Some("POST"), r"/?$", |_req, res| {
        Box::pin(async move {
            res.set_status(201).await;
            res.body("New user created!".to_string()).await;
            Ok(())
        })
    });

    main_router.mount(user_router);

    server.use_middleware(Arc::new(CorsMiddleware));
    server.use_middleware(Arc::new(LoggerMiddleware));

    server.use_middleware(Arc::new(FaviconMiddleware::new(
        "crates/suika_example/public/favicon.ico",
    )));

    server.use_middleware(Arc::new(StaticFileMiddleware::new(
        "/public", "crates/suika_example/public", 3600,
    )));

    server.use_middleware(Arc::new(WasmFileMiddleware::new("/wasm", 86400)));
    server.use_middleware(Arc::new(main_router));

    server.run(None);
}
