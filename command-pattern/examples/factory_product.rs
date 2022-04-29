// -------------------- 产品 ---------------------
pub trait Product {
    fn use_product(&self);
}
#[derive(Clone)]
struct Phone {
    manufacturer: String,
    model: String,
}

// 手机
impl Phone {
    // 实例化
    fn new(manufacturer: String, model: String) -> Phone {
        Phone {
            manufacturer,
            model,
        }
    }
    // 制造商
    fn get_manufacturer(&self) -> String {
        self.manufacturer.clone()
    }
    // 型号
    fn get_model(&self) -> String {
        self.model.clone()
    }
}
// 行为实现
impl Product for Phone {
    fn use_product(&self) {
        println!(
            "Welcome to use {} manufactured by {}",
            self.get_model(),
            self.get_manufacturer()
        );
    }
}
// -------------------- 工厂 ---------------------
trait Factory {
    type Object;
    fn create(&mut self, manufacturer: String, model: String) -> Box<Self::Object> {
        let product = self.create_product(manufacturer, model);
        self.register_product(product)
    }

    fn create_product(&self, manufacturer: String, model: String) -> Box<Self::Object>;
    fn register_product(&mut self, product: Box<Self::Object>) -> Box<Self::Object>;
}

struct PhoneFactory {
    phones: Vec<Phone>,
}
impl PhoneFactory {
    fn new() -> PhoneFactory {
        PhoneFactory { phones: Vec::new() }
    }
}
impl Factory for PhoneFactory {
    type Object = Phone;

    fn create_product(&self, manufacturer: String, model: String) -> Box<Self::Object> {
        Box::new(Phone::new(manufacturer, model))
    }
    fn register_product(&mut self, product: Box<Self::Object>) -> Box<Self::Object> {
        let c = product.clone();
        self.phones.push(*product);
        c
    }
}
// TODO:: 实现其他工厂就可以生成对应的产品了

fn main() {
    let f = PhoneFactory::new();
    let phone1 = f.create_product(String::from("Apple"), String::from("iPhone SE"));
    let phone2 = f.create_product("HUAWEI".to_string(), "Mate 40 Pro".to_string());

    phone1.use_product();
    phone2.use_product();

    println!(
        "Phone1 manufacturer: {}, model: {}",
        phone1.get_manufacturer(),
        phone1.get_model()
    );
    println!(
        "Phone2 manufacturer: {}, model: {}",
        phone2.get_manufacturer(),
        phone2.get_model()
    );
}
