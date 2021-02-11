
extern crate iron;
extern crate router;
#[macro_use] extern crate mime;     // Ici mime est une macro

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serveur en http://localhost:3000...");
    Iron::new(router).http("http://localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {       // On retourne une Ok(response)
    let mut response = Response::new();

    // Construction de la réponse
    response.set_mut(status::Ok);                          // Statut HTTP
    response.set_mut(mime!(Text/Html; Charset=Utf8));      // Défini l'en-tête "Content Type" (Type de média)

    // Réponse : Contenu du corps
    response.set_mut(r#"
        <title>Calculatrice de PGCD</title>
        <form action="/gcd" method="POST">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Calculer le PGCD</button>
        </form>
    "#);

    Ok(response)
}