use crate::{
    contracts::{
        TryFromData,
        token::{TokenKind, TokenRegistry},
        trc20::Trc20Call,
    },
    domain::address::TronAddress,
    listener::subscriber::filters::FilterCtx,
};

pub trait AddressExtractor<T> {
    fn extract(from: T) -> Option<TronAddress>;
}

impl<T> AddressExtractor<T> for () {
    fn extract(_: T) -> Option<TronAddress> {
        None
    }
}

#[derive(Clone)]
pub struct DynamicTrc20Extractor;

impl<R: TokenRegistry> AddressExtractor<FilterCtx<R>>
    for DynamicTrc20Extractor
{
    fn extract(ctx: FilterCtx<R>) -> Option<TronAddress> {
        let token_kind =
            ctx.registry.resolve_token(&ctx.trigger.contract_address)?;

        let data = &Vec::<u8>::from(ctx.trigger.data);

        match token_kind {
            TokenKind::Usdt => {
                let Ok(call) =
                    Trc20Call::<crate::contracts::token::usdt::Usdt>::try_from_data(data)
                else {
                    return None;
                };
                match call {
                    Trc20Call::Transfer(c) => Some(c.recipient),
                    _ => None,
                }
            }
            TokenKind::Usdc => None,
            TokenKind::Other(_) => None,
        }
    }
}
