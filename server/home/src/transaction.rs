use plans::{
    currency::*,
    transaction::Transaction,
};
use seed::{
    *,
    prelude::*,
};
use std::ops::Deref;

pub struct TransactionView<C: 'static + Currency> {
    model: Transaction<C>,
}
impl<C: 'static + Currency> From<Transaction<C>> for TransactionView<C> {
    fn from(transaction: Transaction<C>) -> Self {
        Self {
            model: transaction,
        }
    }
}
impl<C: 'static + Currency> Deref for TransactionView<C> {
    type Target = Transaction<C>;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.model
    }
}
impl<C: 'static + Currency> Default for TransactionView<C> {
    fn default() -> Self {
        Self::from(Transaction::default())
    }
}
pub fn update<C: 'static + Currency>(msg: (), model: &mut TransactionView<C>, _orders: &mut impl Orders<()>) {
}
pub fn view<C: 'static + Currency>(model: &TransactionView<C>) -> impl View<()> {
    tr![class!{"transaction-row"},
        td![class!{"transaction-cell"},
            model.model.get_date().map(|d| format!("{}", d)).unwrap_or("unknown".into())
        ],
        td![class!{"transaction-cell"},
            model.model.get_amount().to_string()
        ],
        td![class!{"transaction-cell"},
            model.model.get_recipient().map(|s| s.to_string()).unwrap_or("None".into())
        ],
        td![class!{"transaction-cell"},
            model.model.get_purposes().map(|ps| ps.to_string()).unwrap_or("None".into())
        ],
    ]
}
