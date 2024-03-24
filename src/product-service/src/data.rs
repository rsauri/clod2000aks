use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Nikhitha".to_string(),
            price: 9.99,
            description: "Nikhitha Angara's Action Figure".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 2,
            name: "Supriya".to_string(),
            price: 9.99,
            description: "Supriya Banala's Action Figure".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 3,
            name: "Piyush".to_string(),
            price: 9.99,
            description: "Piyush Gopalakrishnan Iyer's Action Figure".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 4,
            name: "Swaroop".to_string(),
            price: 9.99,
            description: "Swaroopkrishna Sadasivan Menon's Action Figure".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 5,
            name: "Abhi".to_string(),
            price: 9.99,
            description: "Abhishek Mittal's Action Figure".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 6,
            name: "Ponni".to_string(),
            price: 9.99,
            description: "Ponni Sajeevan's Action Figure".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 7,
            name: "Rose".to_string(),
            price: 9.99,
            description: "Rosemarie Sauri's Action Figure".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 8,
            name: "Dipansu".to_string(),
            price: 9.99,
            description: "Dipansu Dipes Sinha's Action Figure".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 9,
            name: "Maz".to_string(),
            price: 9.99,
            description: "Maziar Sojoudian's Action Figure".to_string(),
            image: "/placeholder.png".to_string()
        }
    ]
}