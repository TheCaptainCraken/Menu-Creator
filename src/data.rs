use rand::{self, seq::IteratorRandom};
use std::fmt::write;
use strum::IntoEnumIterator;
use strum_macros;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Persone {
    Pietro,
    Matteo,
    Achille,
    Silvia,
    Marco,
}

pub enum GiorniSettimana {
    Lunedi,
    Martedi,
    Mercoledi,
    Giovedi,
    Venerdi,
    Sabato,
    Domenica,
}

impl GiorniSettimana {
    pub fn is_feriale(&self) -> bool {
        match self {
            Self::Lunedi => true,
            Self::Martedi => true,
            Self::Mercoledi => true,
            Self::Giovedi => true,
            Self::Venerdi => true,
            Self::Sabato => false,
            Self::Domenica => false,
        }
    }

    pub fn is_weekend(&self) -> bool {
        !self.is_feriale()
    }

    pub fn is_venerdi(&self) -> bool {
        match self {
            Self::Lunedi => false,
            Self::Martedi => false,
            Self::Mercoledi => false,
            Self::Giovedi => false,
            Self::Venerdi => true,
            Self::Sabato => false,
            Self::Domenica => false,
        }
    }
}

fn compatibile(persone: &Vec<Persone>, ricetta: &Vec<Persone>) -> bool {
    for persona in persone {
        if !ricetta.contains(&persona) {
            return false;
        }
    }
    return true;
}

#[derive(Clone, PartialEq, Debug, strum_macros::EnumIter)]
pub enum Ingredienti {
    Gnocchi,
    PassataDiPomodoro,
    Insalata,
    Tonno,
    Formaggio,
    Olive,
    Riso,
    Zafferano,
    Burro,
    Parmigiano,
    MozzarellaDiBufala,
    Mozzarella,
    LasagnaDellEsselunga,
    Pasta,
    SugoAlRaguDiCarne,
    Uova,
    PolloArrosto,
    Patate,
    PanGrattato,
    OlioPerFriggere,
    BrodoVegetale,
    BrodoDiCarne,
    Crostini,
    BastonciniFindus,
    Verdure,
    SpaghettiDiRiso,
    Spinacine,
    Pesto,
    CrocchetteDiPollo,
    PaneDaHamburger,
    Pomodori,
    CordonBleu,
    Spaghetti,
    Melanzane,
    MozzarellaPerPizza,
    PastaSfoglia,
    Pollo,
    Carpaccio,
    Macinato,
    FilettoDiPlatessa,
    Salsicce,
    FruttiDiMare,
    Hamburger,
    Bresaola,
    Vongole,
}

impl std::fmt::Display for Ingredienti {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gnocchi => write(f, format_args!("confezione di gnocchi Rana")),
            Self::PassataDiPomodoro => write(f, format_args!("bottiglia di passata di pomodoro")),
            Self::Insalata => write(f, format_args!("sacchetto di insalata")),
            Self::Tonno => write(f, format_args!("scatoletta di tonno")),
            Self::Formaggio => write(f, format_args!("triangolo di formaggio")),
            Self::Olive => write(f, format_args!("barattolo di olive")),
            Self::Riso => write(f, format_args!("confezione di riso")),
            Self::Zafferano => write(f, format_args!("pacchetto di zafferano")),
            Self::Burro => write(f, format_args!("panetto di burro")),
            Self::Parmigiano => write(f, format_args!("sacchetto di parmigiano grattuggiato")),
            Self::MozzarellaDiBufala => write(f, format_args!("mozzarella di bufala")),
            Self::Mozzarella => write(f, format_args!("mozzarella")),
            Self::LasagnaDellEsselunga => write(f, format_args!("lasagna dell'Esselunga")),
            Self::Pasta => write(f, format_args!("confezione di pasta")),
            Self::SugoAlRaguDiCarne => write(f, format_args!("barattolo di sugo al ragù di carne")),
            Self::Uova => write(f, format_args!("confezione di uova da 6")),
            Self::PolloArrosto => write(f, format_args!("pollo arrosto")),
            Self::Patate => write(f, format_args!("sacco di patate")),
            Self::Pollo => write(f, format_args!("vaschetta di pollo")),
            Self::Carpaccio => write(f, format_args!("vaschetta di carpaccio")),
            Self::Macinato => write(f, format_args!("vaschetta di macinato")),
            Self::FilettoDiPlatessa => write(f, format_args!("vaschetta di filetto di platessa")),
            Self::Salsicce => write(f, format_args!("vaschetta di salsicce")),
            Self::FruttiDiMare => write(f, format_args!("vaschetta di frutti di mare")),
            Self::Hamburger => write(f, format_args!("vaschetta di hamburger")),
            Self::Bresaola => write(f, format_args!("vaschetta di bresaola")),
            Self::Vongole => write(f, format_args!("vaschetta di vongole")),
            Self::PanGrattato => write(f, format_args!("barattolo di pan grattato")),
            Self::OlioPerFriggere => write(f, format_args!("bottiglia di olio per friggere")),
            Self::BrodoVegetale => write(f, format_args!("porzione brodo vegetale")),
            Self::BrodoDiCarne => write(f, format_args!("porzione di brodo di carne")),
            Self::Crostini => write(f, format_args!("porzione di crostini")),
            Self::BastonciniFindus => write(f, format_args!("confezione di bastoncini Findus")),
            Self::Verdure => write(f, format_args!("porzione di verdure")),
            Self::SpaghettiDiRiso => write(f, format_args!("confezione di spaghetti di riso")),
            Self::Spinacine => write(f, format_args!("vaschetta di spinacine")),
            Self::Pesto => write(f, format_args!("porzione di pesto")),
            Self::CrocchetteDiPollo => write(f, format_args!("vaschetta di crocchette di pollo")),
            Self::PaneDaHamburger => write(f, format_args!("confezione di pane da hamburger")),
            Self::Pomodori => write(f, format_args!("sacchetto di pomodori")),
            Self::CordonBleu => write(f, format_args!("vaschetta di cordon bleau")),
            Self::Spaghetti => write(f, format_args!("confezione di spaghetti")),
            Self::Melanzane => write(f, format_args!("melanzana")),
            Self::MozzarellaPerPizza => write(f, format_args!("vaschetta di mozzarella per pizza")),
            Self::PastaSfoglia => write(f, format_args!("confezione di pasta sfoglia")),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Tipo {
    Primo,
    Secondo,
    PiattoUnico,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Ricetta {
    pub nome: String,
    pub ingredienti: Vec<(Ingredienti, f32)>,
    piace_a: Vec<Persone>,
    tipo: Tipo,
    preparabile_in_anticipo: bool,
}

#[derive(Debug, Clone)]
pub struct PresenzeGiorno {
    pub presenze_pranzo: Vec<Persone>,
    pub presenze_cena: Vec<Persone>,
}

#[derive(Debug, Clone)]
pub struct PresenzeSettimana {
    pub presenze_lunedi: PresenzeGiorno,
    pub presenze_martedi: PresenzeGiorno,
    pub presenze_mercoledi: PresenzeGiorno,
    pub presenze_giovedi: PresenzeGiorno,
    pub presenze_venerdi: PresenzeGiorno,
    pub presenze_sabato: PresenzeGiorno,
    pub presenze_domenica: PresenzeGiorno,
}

#[derive(Debug)]
pub struct MenuGiorno {
    pub pranzo: Ricetta,
    pub cena: Ricetta,
}

#[derive(Debug)]
pub struct MenuSettimana {
    pub lunedi: MenuGiorno,
    pub martedi: MenuGiorno,
    pub mercoledi: MenuGiorno,
    pub giovedi: MenuGiorno,
    pub venerdi: MenuGiorno,
    pub sabato: MenuGiorno,
    pub domenica: MenuGiorno,
    pub presenze: PresenzeSettimana,
}

pub fn assembla_menu(presenze: PresenzeSettimana) -> MenuSettimana {
    let copia_presenze = presenze.clone();
    let mut ricette = get_ricette();
    MenuSettimana {
        lunedi: assembla_giorno(
            presenze.presenze_lunedi,
            &mut ricette,
            GiorniSettimana::Lunedi,
        ),
        martedi: assembla_giorno(
            presenze.presenze_martedi,
            &mut ricette,
            GiorniSettimana::Martedi,
        ),
        mercoledi: assembla_giorno(
            presenze.presenze_mercoledi,
            &mut ricette,
            GiorniSettimana::Mercoledi,
        ),
        giovedi: assembla_giorno(
            presenze.presenze_giovedi,
            &mut ricette,
            GiorniSettimana::Giovedi,
        ),
        venerdi: assembla_giorno(
            presenze.presenze_venerdi,
            &mut ricette,
            GiorniSettimana::Venerdi,
        ),
        sabato: assembla_giorno(
            presenze.presenze_sabato,
            &mut ricette,
            GiorniSettimana::Sabato,
        ),
        domenica: assembla_giorno(
            presenze.presenze_domenica,
            &mut ricette,
            GiorniSettimana::Domenica,
        ),
        presenze: copia_presenze,
    }
}

fn assembla_giorno(
    presenze: PresenzeGiorno,
    ricette: &mut Vec<Ricetta>,
    giorno: GiorniSettimana,
) -> MenuGiorno {
    let ricette_pranzo = ricette.iter().filter(|ricetta| {
        if compatibile(&presenze.presenze_pranzo, &ricetta.piace_a)
            && (ricetta.tipo == Tipo::Primo || ricetta.tipo == Tipo::PiattoUnico)
        {
            if giorno.is_feriale() {
                if ricetta.preparabile_in_anticipo == true {
                    true
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            false
        }
    });
    let ricetta_pranzo = match ricette_pranzo.choose(&mut rand::thread_rng()) {
        Some(ricetta) => ricetta.clone(),
        None => Ricetta {
            nome: "Buco".to_string(),
            ingredienti: Vec::new(),
            piace_a: vec![
                Persone::Marco,
                Persone::Matteo,
                Persone::Achille,
                Persone::Silvia,
                Persone::Pietro,
            ],
            tipo: Tipo::PiattoUnico,
            preparabile_in_anticipo: true,
        },
    };
    if ricetta_pranzo.nome != "Buco" {
        ricette.remove(
            ricette
                .iter()
                .position(|ricetta| ricetta.nome == ricetta_pranzo.nome)
                .expect("Recipe Should be in the vector."),
        );
    }

    if !giorno.is_venerdi() {
        let ricette_cena = ricette.iter().filter(|ricetta| {
            if compatibile(&presenze.presenze_cena, &ricetta.piace_a)
                && (ricetta.tipo == Tipo::Secondo || ricetta.tipo == Tipo::PiattoUnico)
            {
                true
            } else {
                false
            }
        });
        let ricetta_cena = match ricette_cena.choose(&mut rand::thread_rng()) {
            Some(ricetta) => ricetta.clone(),
            None => Ricetta {
                nome: "Buco".to_string(),
                ingredienti: Vec::new(),
                piace_a: vec![
                    Persone::Marco,
                    Persone::Matteo,
                    Persone::Achille,
                    Persone::Silvia,
                    Persone::Pietro,
                ],
                tipo: Tipo::PiattoUnico,
                preparabile_in_anticipo: true,
            },
        };
        if ricetta_cena.nome != "Buco" {
            ricette.remove(
                ricette
                    .iter()
                    .position(|ricetta| ricetta.nome == ricetta_cena.nome)
                    .expect("Recipe Should be in the vector."),
            );
        }
        return MenuGiorno {
            pranzo: ricetta_pranzo,
            cena: ricetta_cena,
        };
    } else {
        return MenuGiorno {
            pranzo: ricetta_pranzo,
            cena: Ricetta {
                nome: "Asporto".to_string(),
                ingredienti: Vec::new(),
                piace_a: vec![
                    Persone::Marco,
                    Persone::Matteo,
                    Persone::Achille,
                    Persone::Silvia,
                    Persone::Pietro,
                ],
                tipo: Tipo::PiattoUnico,
                preparabile_in_anticipo: true,
            },
        };
    }
}

pub fn get_ricette() -> Vec<Ricetta> {
    vec![
        Ricetta {
            nome: "Gnocchi al pomodoro".to_string(),
            ingredienti: vec![
                (Ingredienti::Gnocchi, 0.3),
                (Ingredienti::PassataDiPomodoro, 0.3),
            ],
            piace_a: vec![Persone::Pietro, Persone::Achille, Persone::Silvia],
            tipo: Tipo::Primo,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Insalata".to_string(),
            ingredienti: vec![
                (Ingredienti::Insalata, 1.0),
                (Ingredienti::Tonno, 1.0),
                (Ingredienti::Formaggio, 0.2),
                (Ingredienti::Olive, 0.3),
            ],
            piace_a: vec![Persone::Silvia, Persone::Marco, Persone::Achille],
            tipo: Tipo::PiattoUnico,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Risotto allo zafferano".to_string(),
            ingredienti: vec![
                (Ingredienti::Riso, 0.2),
                (Ingredienti::Burro, 0.1),
                (Ingredienti::Zafferano, 0.2),
                (Ingredienti::Parmigiano, 0.2),
            ],
            piace_a: vec![
                Persone::Silvia,
                Persone::Marco,
                Persone::Achille,
                Persone::Pietro,
                Persone::Matteo,
            ],
            tipo: Tipo::Primo,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Mozzarelle".to_string(),
            ingredienti: vec![
                (Ingredienti::Mozzarella, 0.5),
                (Ingredienti::MozzarellaDiBufala, 0.5),
            ],
            piace_a: vec![Persone::Marco, Persone::Achille, Persone::Matteo],
            tipo: Tipo::Secondo,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Parmigiana".to_string(),
            ingredienti: vec![
                (Ingredienti::Melanzane, 0.3),
                (Ingredienti::MozzarellaPerPizza, 0.2),
                (Ingredienti::PassataDiPomodoro, 0.1),
            ],
            piace_a: vec![Persone::Marco, Persone::Matteo],
            tipo: Tipo::PiattoUnico,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Lasagne".to_string(),
            ingredienti: vec![(Ingredienti::LasagnaDellEsselunga, 1.0)],
            piace_a: vec![
                Persone::Marco,
                Persone::Matteo,
                Persone::Pietro,
                Persone::Achille,
            ],
            tipo: Tipo::PiattoUnico,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Pasta al ragù di carne".to_string(),
            ingredienti: vec![
                (Ingredienti::Pasta, 0.2),
                (Ingredienti::SugoAlRaguDiCarne, 0.2),
            ],
            piace_a: vec![Persone::Marco, Persone::Matteo, Persone::Achille],
            tipo: Tipo::Primo,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Pasta al tonno".to_string(),
            ingredienti: vec![(Ingredienti::Pasta, 0.2), (Ingredienti::Tonno, 1.0)],
            piace_a: vec![Persone::Marco, Persone::Matteo, Persone::Achille],
            tipo: Tipo::Primo,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Uova strapazzate".to_string(),
            ingredienti: vec![(Ingredienti::Uova, 0.33)],
            piace_a: vec![Persone::Matteo, Persone::Achille, Persone::Pietro],
            tipo: Tipo::Secondo,
            preparabile_in_anticipo: false,
        },
        Ricetta {
            nome: "Frittata con le zucchine".to_string(),
            ingredienti: vec![(Ingredienti::Uova, 0.3)],
            piace_a: vec![
                Persone::Marco,
                Persone::Matteo,
                Persone::Achille,
                Persone::Pietro,
            ],
            tipo: Tipo::Secondo,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Pollo arrosto e patatine".to_string(),
            ingredienti: vec![(Ingredienti::PolloArrosto, 0.2), (Ingredienti::Patate, 0.2)],
            piace_a: vec![
                Persone::Marco,
                Persone::Matteo,
                Persone::Achille,
                Persone::Pietro,
            ],
            tipo: Tipo::PiattoUnico,
            preparabile_in_anticipo: false,
        },
        Ricetta {
            nome: "Cotolette e zucchine".to_string(),
            ingredienti: vec![
                (Ingredienti::Pollo, 0.5),
                (Ingredienti::Uova, 0.2),
                (Ingredienti::PanGrattato, 0.2),
                (Ingredienti::OlioPerFriggere, 0.2),
            ],
            piace_a: vec![
                Persone::Marco,
                Persone::Matteo,
                Persone::Achille,
                Persone::Pietro,
            ],
            tipo: Tipo::PiattoUnico,
            preparabile_in_anticipo: false,
        },
        Ricetta {
            nome: "Passato di verdure".to_string(),
            ingredienti: vec![(Ingredienti::Verdure, 1.0), (Ingredienti::Crostini, 1.0)],
            piace_a: vec![Persone::Marco, Persone::Matteo, Persone::Silvia],
            tipo: Tipo::Primo,
            preparabile_in_anticipo: false,
        },
        Ricetta {
            nome: "Carnina".to_string(),
            ingredienti: vec![(Ingredienti::Carpaccio, 0.5)],
            piace_a: vec![
                Persone::Marco,
                Persone::Matteo,
                Persone::Pietro,
                Persone::Achille,
            ],
            tipo: Tipo::Secondo,
            preparabile_in_anticipo: false,
        },
        Ricetta {
            nome: "Bastoncini Findus".to_string(),
            ingredienti: vec![(Ingredienti::BastonciniFindus, 0.33)],
            piace_a: vec![
                Persone::Marco,
                Persone::Matteo,
                Persone::Pietro,
                Persone::Achille,
            ],
            tipo: Tipo::Secondo,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Spaghetti di riso con verdure".to_string(),
            ingredienti: vec![
                (Ingredienti::SpaghettiDiRiso, 0.3),
                (Ingredienti::Verdure, 1.0),
            ],
            piace_a: vec![Persone::Matteo, Persone::Silvia, Persone::Marco],
            tipo: Tipo::Primo,
            preparabile_in_anticipo: false,
        },
        Ricetta {
            nome: "Spinacine".to_string(),
            ingredienti: vec![(Ingredienti::Spinacine, 1.0)],
            piace_a: vec![
                Persone::Matteo,
                Persone::Marco,
                Persone::Achille,
                Persone::Pietro,
            ],
            tipo: Tipo::Secondo,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Polpette di carne".to_string(),
            ingredienti: vec![
                (Ingredienti::Macinato, 0.2),
                (Ingredienti::PanGrattato, 0.2),
                (Ingredienti::PassataDiPomodoro, 0.3),
            ],
            piace_a: vec![
                Persone::Matteo,
                Persone::Marco,
                Persone::Pietro,
                Persone::Achille,
            ],
            tipo: Tipo::Secondo,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Pasta al pesto".to_string(),
            ingredienti: vec![(Ingredienti::Pasta, 0.2), (Ingredienti::Pesto, 0.2)],
            piace_a: vec![
                Persone::Matteo,
                Persone::Marco,
                Persone::Pietro,
                Persone::Achille,
                Persone::Silvia,
            ],
            tipo: Tipo::Primo,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Platessa al forno".to_string(),
            ingredienti: vec![
                (Ingredienti::PanGrattato, 0.2),
                (Ingredienti::FilettoDiPlatessa, 0.3),
            ],
            piace_a: vec![
                Persone::Matteo,
                Persone::Marco,
                Persone::Pietro,
                Persone::Achille,
            ],
            tipo: Tipo::Secondo,
            preparabile_in_anticipo: false,
        },
        Ricetta {
            nome: "Crocchette di pollo".to_string(),
            ingredienti: vec![(Ingredienti::CrocchetteDiPollo, 0.33)],
            piace_a: vec![
                Persone::Matteo,
                Persone::Marco,
                Persone::Pietro,
                Persone::Achille,
            ],
            tipo: Tipo::Secondo,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Salsicce al pomodoro".to_string(),
            ingredienti: vec![(Ingredienti::Salsicce, 0.33)],
            piace_a: vec![
                Persone::Matteo,
                Persone::Marco,
                Persone::Pietro,
                Persone::Achille,
            ],
            tipo: Tipo::Secondo,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Riso ai frutti di mare".to_string(),
            ingredienti: vec![(Ingredienti::Riso, 0.2), (Ingredienti::FruttiDiMare, 1.0)],
            piace_a: vec![Persone::Matteo, Persone::Marco, Persone::Achille],
            tipo: Tipo::PiattoUnico,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Hamburger di carne".to_string(),
            ingredienti: vec![
                (Ingredienti::Hamburger, 0.25),
                (Ingredienti::Formaggio, 0.1),
                (Ingredienti::Pomodori, 0.1),
                (Ingredienti::PaneDaHamburger, 0.15),
            ],
            piace_a: vec![
                Persone::Matteo,
                Persone::Marco,
                Persone::Achille,
                Persone::Pietro,
            ],
            tipo: Tipo::PiattoUnico,
            preparabile_in_anticipo: false,
        },
        Ricetta {
            nome: "Polepttone".to_string(),
            ingredienti: vec![
                (Ingredienti::Macinato, 0.2),
                (Ingredienti::Uova, 0.2),
                (Ingredienti::Parmigiano, 1.0),
            ],
            piace_a: vec![
                Persone::Matteo,
                Persone::Marco,
                Persone::Achille,
                Persone::Pietro,
            ],
            tipo: Tipo::Secondo,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Cordon bleu".to_string(),
            ingredienti: vec![(Ingredienti::CordonBleu, 1.0)],
            piace_a: vec![
                Persone::Matteo,
                Persone::Marco,
                Persone::Achille,
                Persone::Pietro,
            ],
            tipo: Tipo::Secondo,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Bresaola".to_string(),
            ingredienti: vec![(Ingredienti::Bresaola, 1.0)],
            piace_a: vec![
                Persone::Matteo,
                Persone::Marco,
                Persone::Achille,
                Persone::Pietro,
            ],
            tipo: Tipo::Secondo,
            preparabile_in_anticipo: true,
        },
        Ricetta {
            nome: "Spaghetti alle vongole".to_string(),
            ingredienti: vec![(Ingredienti::Vongole, 1.0), (Ingredienti::Spaghetti, 0.2)],
            piace_a: vec![
                Persone::Matteo,
                Persone::Marco,
                Persone::Achille,
                Persone::Pietro,
            ],
            tipo: Tipo::PiattoUnico,
            preparabile_in_anticipo: false,
        },
        Ricetta {
            nome: "Torta salata".to_string(),
            ingredienti: vec![
                (Ingredienti::Verdure, 1.0),
                (Ingredienti::Mozzarella, 1.0),
                (Ingredienti::Parmigiano, 0.2),
                (Ingredienti::PastaSfoglia, 0.1),
            ],
            piace_a: vec![Persone::Matteo, Persone::Marco, Persone::Silvia],
            tipo: Tipo::PiattoUnico,
            preparabile_in_anticipo: true,
        },
    ]
}

pub fn genera_lista_della_spesa(menu: MenuSettimana) -> String {
    let mut ingredienti = Vec::new();

    ingredienti.push((
        menu.lunedi.pranzo.ingredienti.clone(),
        menu.presenze.presenze_lunedi.presenze_pranzo.len(),
    ));
    ingredienti.push((
        menu.lunedi.cena.ingredienti.clone(),
        menu.presenze.presenze_lunedi.presenze_cena.len(),
    ));

    ingredienti.push((
        menu.martedi.pranzo.ingredienti.clone(),
        menu.presenze.presenze_martedi.presenze_pranzo.len(),
    ));
    ingredienti.push((
        menu.martedi.cena.ingredienti.clone(),
        menu.presenze.presenze_martedi.presenze_cena.len(),
    ));

    ingredienti.push((
        menu.mercoledi.pranzo.ingredienti.clone(),
        menu.presenze.presenze_mercoledi.presenze_pranzo.len(),
    ));
    ingredienti.push((
        menu.mercoledi.cena.ingredienti.clone(),
        menu.presenze.presenze_mercoledi.presenze_cena.len(),
    ));

    ingredienti.push((
        menu.giovedi.pranzo.ingredienti.clone(),
        menu.presenze.presenze_giovedi.presenze_pranzo.len(),
    ));
    ingredienti.push((
        menu.giovedi.cena.ingredienti.clone(),
        menu.presenze.presenze_giovedi.presenze_cena.len(),
    ));

    ingredienti.push((
        menu.venerdi.pranzo.ingredienti.clone(),
        menu.presenze.presenze_venerdi.presenze_pranzo.len(),
    ));
    ingredienti.push((
        menu.venerdi.cena.ingredienti.clone(),
        menu.presenze.presenze_venerdi.presenze_cena.len(),
    ));

    ingredienti.push((
        menu.sabato.pranzo.ingredienti.clone(),
        menu.presenze.presenze_sabato.presenze_pranzo.len(),
    ));
    ingredienti.push((
        menu.sabato.cena.ingredienti.clone(),
        menu.presenze.presenze_sabato.presenze_cena.len(),
    ));

    ingredienti.push((
        menu.domenica.pranzo.ingredienti.clone(),
        menu.presenze.presenze_domenica.presenze_pranzo.len(),
    ));
    ingredienti.push((
        menu.domenica.cena.ingredienti.clone(),
        menu.presenze.presenze_domenica.presenze_cena.len(),
    ));

    let mut lista_della_spesa = String::new();

    for ingrediente in Ingredienti::iter() {
        let mut counter = 0.0;

        for (ricetta, persone) in &ingredienti {
            for (ingre, qta) in ricetta {
                if ingre == &ingrediente {
                    counter += qta * *persone as f32;
                }
            }
        }

        if counter > 0.0 {
            lista_della_spesa
                .push_str(format!("- {} × {}\n", counter.ceil() as i32, ingrediente).as_str());
        }
    }

    lista_della_spesa
}
