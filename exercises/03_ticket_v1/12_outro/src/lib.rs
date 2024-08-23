// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        let product_name = check_product_name(product_name).unwrap();
        let quantity = check_greater_zero(quantity).unwrap();
        let unit_price = check_greater_zero(unit_price).unwrap();
        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn set_product_name(&mut self, new_product_name: String) {
        let new_product_name = check_product_name(new_product_name).unwrap();
        self.product_name = new_product_name
    }

    pub fn set_quantity(&mut self, new_quantity: u32) {
        let new_quantity = check_greater_zero(new_quantity).unwrap();
        self.quantity = new_quantity
    }

    pub fn set_unit_price(&mut self, new_unit_price: u32) {
        let new_unit_price = check_greater_zero(new_unit_price).unwrap();
        self.unit_price = new_unit_price
    }
}

fn check_product_name(product_name: String) -> Result<String, String> {
    if product_name.is_empty() {
        Err("product_name can't be empty".to_string())
    } else if product_name.len() > 300 {
        Err("product_name's length can't larger than 300".to_string())
    } else {
        Ok(product_name)
    }
}

fn check_greater_zero(number: u32) -> Result<u32, String> {
    if number <= 0 {
        Err("this number must greater than 0".to_string())
    } else {
        Ok(number)
    }
}
