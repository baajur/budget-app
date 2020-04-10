use seed::{
    *,
    prelude::*,
};

use plans::{
    *,
    currency::*,
};
use crate::{
    *,
    transaction::*,
};
use std::ops::Deref;

pub struct TransactionsView<C: 'static + Currency> {
    model: Transactions<C>,
}
impl<C: 'static + Currency> Deref for TransactionsView<C> {
    type Target = Transactions<C>;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.model
    }
}
impl<C: 'static + Currency> From<Transactions<C>> for TransactionsView<C> {
    fn from(transactions: Transactions<C>) -> Self {
        Self {
            model: transactions,
        }
    }
}
impl<C: 'static + Currency> Default for TransactionsView<C> {
    fn default() -> Self {
        Self::from(Transactions::from(Vec::new()))
    }
}
pub fn update<C: 'static + Currency>(msg: (), model: &mut TransactionsView<C>, _orders: &mut impl Orders<()>) {
}
pub fn view<C: 'static + Currency>(model: &TransactionsView<C>) -> impl View<()> {
    table![class!{"transaction-table"},
        caption![class!{"transaction-caption"},
            "Your Transactions"
        ],
        tr![class!{"transaction-row"},
            th![class!{"transaction-header"}, "Date" ],
            th![class!{"transaction-header"}, "Amount" ],
            th![class!{"transaction-header"}, "Partner" ],
            th![class!{"transaction-header"}, "Purposes" ],
        ],
        model.model.iter().map(|t|
            div![
                TransactionView::from(t.clone())
            ]
        )
    ]
}
