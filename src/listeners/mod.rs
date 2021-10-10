mod ready;

use poise::{BoxFuture, Event, Framework};
use ready::ready;

use crate::{AkasukiError, AkasukiResult};

pub fn listener<'a, A>(
    ctx: &'a serenity::prelude::Context,
    event: &'a Event,
    _framework: &'a Framework<A, AkasukiError>,
    _user_data: &'a A,
) -> BoxFuture<'a, AkasukiResult<()>> {
    match event {
        Event::Ready { data_about_bot } => Box::pin(ready(ctx, data_about_bot)),
        _ => todo!(),
    }
}
