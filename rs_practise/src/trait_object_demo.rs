trait Sale {
    fn amount(&self) -> f64;
}

struct FullSale(f64);
impl Sale for FullSale {
    fn amount(&self) -> f64 {
        self.0
    }
}
struct OneDollarOffCoupon(f64);
impl Sale for OneDollarOffCoupon {
    fn amount(&self) -> f64 {
        self.0 - 1.0
    }
}
struct TenPercentOffPromotion(f64);
impl Sale for TenPercentOffPromotion {
    fn amount(&self) -> f64 {
        self.0 * 0.9
    }
}

fn calculate_revenue(sales: &Vec<Box<dyn Sale>>) -> f64 {
    sales.iter().map(|sale| sale.amount()).sum()
}

pub fn trait_object_demo() {
    let price = 20.0;
    let regular_sale = Box::new(FullSale(price));
    let coupon_sale = Box::new(OneDollarOffCoupon(price));
    let promotion_sale = Box::new(TenPercentOffPromotion(price));
    // 这里需要手动指定类型，因为Sale是trait，不能直接使用
    let sales: Vec<Box<dyn Sale>> = vec![regular_sale, coupon_sale, promotion_sale];
    println!("revenue: {}", calculate_revenue(&sales));
}
