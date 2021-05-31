// Title: Froggy The Chatterbox

fn main() {
    let text = "?em pleh esaelp uoy naC !oot ,gnis I semitemos dnA .klat dna klat dna klat I :raelc si rewsna ehT ?em ot naem os elpoep era yhW .elopdat elttil a tsuj saw I nehw em dnuof eH .gorf etirovaf s'draziw eht ma I tuB ?niaga revo dna revo em esaet elpoep od yhW .llaw eht tsniaga nworht saw ohw gorf elttil roop a ma I";
    let text = text.chars().rev().collect::<String>();

    println!("{}", text);
}
