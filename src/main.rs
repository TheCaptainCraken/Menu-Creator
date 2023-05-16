use prettytable;

pub mod data;
fn main() {
    let presenze = data::PresenzeSettimana {
        presenze_lunedi: data::PresenzeGiorno {
            presenze_pranzo: vec![
                data::Persone::Matteo,
                data::Persone::Achille,
                data::Persone::Marco,
            ],
            presenze_cena: vec![
                data::Persone::Matteo,
                data::Persone::Achille,
                data::Persone::Pietro,
                data::Persone::Marco,
            ],
        },
        presenze_martedi: data::PresenzeGiorno {
            presenze_pranzo: vec![
                data::Persone::Matteo,
                data::Persone::Achille,
                data::Persone::Marco,
            ],
            presenze_cena: vec![
                data::Persone::Matteo,
                data::Persone::Achille,
                data::Persone::Pietro,
                data::Persone::Marco,
            ],
        },
        presenze_mercoledi: data::PresenzeGiorno {
            presenze_pranzo: vec![
                data::Persone::Matteo,
                data::Persone::Achille,
                data::Persone::Marco,
            ],
            presenze_cena: vec![
                data::Persone::Matteo,
                data::Persone::Achille,
                data::Persone::Pietro,
                data::Persone::Marco,
            ],
        },
        presenze_giovedi: data::PresenzeGiorno {
            presenze_pranzo: vec![
                data::Persone::Matteo,
                data::Persone::Achille,
                data::Persone::Marco,
            ],
            presenze_cena: vec![
                data::Persone::Matteo,
                data::Persone::Achille,
                data::Persone::Pietro,
                data::Persone::Marco,
            ],
        },
        presenze_venerdi: data::PresenzeGiorno {
            presenze_pranzo: vec![
                data::Persone::Matteo,
                data::Persone::Achille,
                data::Persone::Marco,
            ],
            presenze_cena: vec![
                data::Persone::Matteo,
                data::Persone::Achille,
                data::Persone::Pietro,
                data::Persone::Marco,
            ],
        },
        presenze_sabato: data::PresenzeGiorno {
            presenze_pranzo: vec![
                data::Persone::Matteo,
                data::Persone::Achille,
                data::Persone::Pietro,
                data::Persone::Marco,
            ],
            presenze_cena: vec![
                data::Persone::Matteo,
                data::Persone::Achille,
                data::Persone::Pietro,
                data::Persone::Marco,
            ],
        },
        presenze_domenica: data::PresenzeGiorno {
            presenze_pranzo: vec![
                data::Persone::Matteo,
                data::Persone::Achille,
                data::Persone::Pietro,
                data::Persone::Marco,
            ],
            presenze_cena: vec![
                data::Persone::Matteo,
                data::Persone::Achille,
                data::Persone::Pietro,
                data::Persone::Marco,
            ],
        },
    };

    let menu = data::assembla_menu(presenze);

    let mut rendered_menu = prettytable::Table::new();

    rendered_menu.add_row(prettytable::row![b -> "Giorno", bc -> "Pranzo", bc -> "Cena"]);
    rendered_menu.add_row(prettytable::row![
        bFg -> "Lunedì",
        menu.lunedi.pranzo.nome,
        menu.lunedi.cena.nome,
    ]);
    rendered_menu.add_row(prettytable::row![
        bFg -> "Martedì",
        menu.martedi.pranzo.nome,
        menu.martedi.cena.nome,
    ]);
    rendered_menu.add_row(prettytable::row![
        bFg -> "Mercoledì",
        menu.mercoledi.pranzo.nome,
        menu.mercoledi.cena.nome,
    ]);
    rendered_menu.add_row(prettytable::row![
        bFg -> "Giovedì",
        menu.giovedi.pranzo.nome,
        menu.giovedi.cena.nome,
    ]);
    rendered_menu.add_row(prettytable::row![
        bFg -> "Venerdì",
        menu.venerdi.pranzo.nome,
        menu.venerdi.cena.nome,
    ]);
    rendered_menu.add_row(prettytable::row![
        bFg -> "sabato",
        menu.sabato.pranzo.nome,
        menu.sabato.cena.nome,
    ]);
    rendered_menu.add_row(prettytable::row![
        bFg -> "Domenica",
        menu.domenica.pranzo.nome,
        menu.domenica.cena.nome,
    ]);
    rendered_menu.printstd();
    println!("");
    println!("Lista della spesa");
    println!("=================");
    println!("{}", data::genera_lista_della_spesa(menu));
}
