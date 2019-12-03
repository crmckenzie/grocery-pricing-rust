use crate::*;

#[test]
pub fn one_item() {
    let cart = Cart {
        items: vec![Product {
            quantity: 1,
            unit_price: 200,
            sale: None,
        }],
    };

    assert_eq!(cart.calculate_net_total(), 200);
}

#[test]
pub fn two_items() {
    let cart = Cart {
        items: vec![
            Product {
                quantity: 1,
                unit_price: 200,
                sale: None,
            },
            Product {
                quantity: 1,
                unit_price: 100,
                sale: None,
            },
        ],
    };

    assert_eq!(cart.calculate_net_total(), 300);
}

#[test]
pub fn two_of_one_item() {
    let cart = Cart {
        items: vec![Product {
            quantity: 2,
            unit_price: 200,
            sale: None,
        }],
    };

    assert_eq!(cart.calculate_net_total(), 400);
}

#[test]
pub fn buy_2_get_1_free_qty_1() {
    let cart = Cart {
        items: vec![Product {
            quantity: 1,
            unit_price: 200,
            sale: Some(Sale {
                sale_quantity: 2,
                free_quantity: 1,
            }),
        }],
    };

    assert_eq!(cart.calculate_net_total(), 200);
}

#[test]
pub fn buy_2_get_1_free_qty_2() {
    let cart = Cart {
        items: vec![Product {
            quantity: 2,
            unit_price: 200,
            sale: Some(Sale {
                sale_quantity: 2,
                free_quantity: 1,
            }),
        }],
    };

    assert_eq!(cart.calculate_net_total(), 400);
}

#[test]
pub fn buy_2_get_1_free_qty_3() {
    let cart = Cart {
        items: vec![Product {
            quantity: 3,
            unit_price: 200,
            sale: Some(Sale {
                sale_quantity: 2,
                free_quantity: 1,
            }),
        }],
    };

    assert_eq!(cart.calculate_net_total(), 400);
}

#[test]
pub fn buy_2_get_1_free_qty_4() {
    let cart = Cart {
        items: vec![Product {
            quantity: 4,
            unit_price: 200,
            sale: Some(Sale {
                sale_quantity: 2,
                free_quantity: 1,
            }),
        }],
    };

    assert_eq!(cart.calculate_net_total(), 600);
}

#[test]
pub fn buy_2_get_1_free_qty_5() {
    let cart = Cart {
        items: vec![Product {
            quantity: 5,
            unit_price: 200,
            sale: Some(Sale {
                sale_quantity: 2,
                free_quantity: 1,
            }),
        }],
    };

    assert_eq!(cart.calculate_net_total(), 800);
}

#[test]
pub fn buy_2_get_1_free_qty_6() {
    let cart = Cart {
        items: vec![Product {
            quantity: 6,
            unit_price: 200,
            sale: Some(Sale {
                sale_quantity: 2,
                free_quantity: 1,
            }),
        }],
    };

    assert_eq!(cart.calculate_net_total(), 800);
}

#[test]
pub fn buy_3_get_2_free_qty_1() {
    let cart = Cart {
        items: vec![Product {
            quantity: 1,
            unit_price: 200,
            sale: Some(Sale {
                sale_quantity: 3,
                free_quantity: 2,
            }),
        }],
    };

    assert_eq!(cart.calculate_net_total(), 200);
}

#[test]
pub fn buy_3_get_2_free_qty_2() {
    let cart = Cart {
        items: vec![Product {
            quantity: 2,
            unit_price: 200,
            sale: Some(Sale {
                sale_quantity: 3,
                free_quantity: 2,
            }),
        }],
    };

    assert_eq!(cart.calculate_net_total(), 400);
}

#[test]
pub fn buy_3_get_2_free_qty_3() {
    let cart = Cart {
        items: vec![Product {
            quantity: 3,
            unit_price: 200,
            sale: Some(Sale {
                sale_quantity: 3,
                free_quantity: 2,
            }),
        }],
    };

    assert_eq!(cart.calculate_net_total(), 600);
}

#[test]
pub fn buy_3_get_2_free_qty_4() {
    let cart = Cart {
        items: vec![Product {
            quantity: 4,
            unit_price: 200,
            sale: Some(Sale {
                sale_quantity: 3,
                free_quantity: 2,
            }),
        }],
    };

    assert_eq!(cart.calculate_net_total(), 600);
}

#[test]
pub fn buy_3_get_2_free_qty_5() {
    let cart = Cart {
        items: vec![Product {
            quantity: 5,
            unit_price: 200,
            sale: Some(Sale {
                sale_quantity: 3,
                free_quantity: 2,
            }),
        }],
    };

    assert_eq!(cart.calculate_net_total(), 600);
}
#[test]
pub fn buy_3_get_2_free_qty_6() {
    let cart = Cart {
        items: vec![Product {
            quantity: 6,
            unit_price: 200,
            sale: Some(Sale {
                sale_quantity: 3,
                free_quantity: 2,
            }),
        }],
    };

    assert_eq!(cart.calculate_net_total(), 800);
}