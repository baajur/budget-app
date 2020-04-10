pub use plans::{
    *,
    currency::*,
};
use seed::{
    *,
    prelude::*,
};

use crate::{
    *,
    transactions::*,
};

pub enum Msg {
    Update,
}

pub struct BudgetView<C: 'static + Currency> {
    model: Budget<C>,
}
impl<C: 'static + Currency> Default for BudgetView<C> {
    fn default() -> Self {
        let mut b = Budget::create("My Budget", 0);
        b.get(100).add_purpose("Money");
        b.get(100).add_purpose("Money");
        b.get(100).add_purpose("Money");
        Self {
            model: b,
        }
    }
}
pub fn update<C: 'static + Currency>(msg: Msg, model: &mut BudgetView<C>, _orders: &mut impl Orders<Msg>) {
}
pub fn view<C: 'static + Currency>(model: &BudgetView<C>) -> impl View<Msg> {
    div![class!{"budget-container"},
        h1![class!{"budget-heading"},
            model.model.name()
        ],
        TransactionsView::from(model.model.transactions.clone())
    ]
}
//pub struct BudgetView<C: 'static + Currency> {
//    link: ComponentLink<Self>,
//    model: Budget<C>,
//}
//
//impl<C: 'static + Currency> Component for BudgetView<C> {
//    type Message = Msg;
//    type Properties = ();
//
//    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
//        let mut b = Budget::create("My Budget", 0);
//        b.get(100).add_purpose("Money");
//        b.get(100).add_purpose("Money");
//        b.get(100).add_purpose("Money");
//        Self {
//            link,
//            model: b,
//        }
//    }
//    fn view(&self) -> Html {
//        html!{
//            <div class="budget-container">
//                <h1 class="budget-heading">{self.model.name()}</h1>
//                {TransactionsView::from(self.model.transactions.clone()).view()}
//            </div>
//        }
//    }
//    fn update(&mut self, msg: Self::Message) -> ShouldRender {
//        match msg {
//            Msg::Update => {
//                true
//            },
//            _ => false
//        }
//    }
//}
