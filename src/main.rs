
extern crate iron;
extern crate router;
extern crate urlencoded;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;


fn main() {
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serveur en http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {       // On retourne une Ok(response)
    let mut response = Response::new();

    // Construction de la réponse
    response.set_mut(status::Ok);                          // Statut HTTP
    response.set_mut(mime!(Text/Html; Charset=Utf8));      // Défini l'en-tête "Content Type" (Type de média)

    // Réponse : Contenu du corps
    response.set_mut(r#"
        <title>Calculatrice de PGCD</title>
        <h1>Calculatrice de PGCD</h1>
        <p>Usage : entrez deux nombres et lancez le calcul</p>
        <form action="/gcd" method="POST">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Calculer le PGCD</button>
        </form>
    "#);

    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Erreur analyse formulaire: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };
    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Pas de 'n' dans le formulaire\n"));
            return Ok(response);
        }
        Some(nums) => nums
    };

    let mut nombres = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(
                    format!("Valeur non num. pour 'n': {:?}",unparsed));
                return Ok(response);
            }
            Ok(n) => { nombres.push(n); }
        }
    }

    let mut d = nombres[0];
    for m in &nombres[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        format!("Le PGCD de {:?} est <b>{}</b>\n", nombres, d)
    );

    Ok(response)
}

fn gcd(mut n: u64, mut m:u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}