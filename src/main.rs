 #[cfg(test)]
mod tests;

pub struct Sale {
    sale_quantity: u32,
    free_quantity: u32,
}

pub struct ZeroNotAllowed { field: String}

impl Sale {

    fn new(sale_quantity: u32, free_quantity: u32) -> Result<Sale, ZeroNotAllowed> {
        if sale_quantity == 0 {
            return Err(ZeroNotAllowed { field: "sale_quantity".to_string()});
        }

        if free_quantity == 0 {
            return Err(ZeroNotAllowed { field: "free_quantity".to_string()});
        }

        Ok(Sale {
            sale_quantity,
            free_quantity,
        })
    }

    fn calculate_net_total(&self, item: &Product) -> u32 {
        // complete sets
        let set_size = self.sale_quantity + self.free_quantity;
        let set_count = item.quantity / set_size;
        let quantity_free = set_count * self.free_quantity;
        let mut discount = quantity_free * item.unit_price;

        // partial sets
        let remaining_quantity = item.quantity - (set_size * set_count);
        if remaining_quantity > self.sale_quantity {
            let remaining_free = remaining_quantity - self.sale_quantity;
            discount = discount + remaining_free * item.unit_price;
        }

        let total = (item.unit_price * item.quantity) - discount;
        total
    }
}

pub struct Product {
    quantity: u32,
    unit_price: u32,
    sale: Option<Sale>,
}

impl Product {
    fn calculate_net_total(&self) -> u32 {
        match &self.sale {
            Some(sale) => sale.calculate_net_total(self),
            None => self.quantity * self.unit_price,
        }
    }
}

pub struct Cart {
    items: Vec<Product>,
}

impl Cart {
    pub fn calculate_net_total(&self) -> u32 {
        let mut total = 0;
        for item in &self.items {
            total = total + item.calculate_net_total();
        }
        total
    }
}

fn main() {
    println!("Hello, world!");
}
