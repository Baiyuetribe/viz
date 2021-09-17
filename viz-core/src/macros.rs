use std::future::Future;

use viz_utils::futures::future::BoxFuture;

use crate::{Context, Error, Extract, HandlerBase, HandlerCamp, Response, Result};

macro_rules! peel {
    ($T0:ident, $($T:ident,)*) => (tuple! { $($T,)* })
}

macro_rules! tuple {
    () => (
        #[doc(hidden)]
        impl Extract for () {
            type Error = Error;

            #[inline]
            fn extract(_: &mut Context) -> BoxFuture<'_, Result<Self, Self::Error>> {
                Box::pin(async { Ok(()) })
            }
        }

        #[doc(hidden)]
        impl<F, R> HandlerBase<()> for F
        where
            F: Fn() -> R + Clone + 'static,
            R: Future + Send + 'static,
            R::Output: Into<Response>,
        {
            type Output = R::Output;
            type Future = R;

            #[inline]
            fn call(&self, _: ()) -> R {
                (self)()
            }
        }

        #[doc(hidden)]
        impl<'h, F, R> HandlerCamp<'h, ()> for F
        where
            F: Fn(&'h mut Context) -> R + Clone + 'static,
            R: Future + Send + 'h,
            R::Output: Into<Response>,
        {
            type Output = R::Output;
            type Future = R;

            #[inline]
            fn call(&'h self, cx: &'h mut Context, _: ()) -> R {
                (self)(cx)
            }
        }
    );
    ($($T:ident,)+) => (
        impl<$($T),+> Extract for ($($T,)+)
        where
            $($T: Extract + Send,)+
            $($T::Error: Into<Response> + Send + 'static,)+
        {
            type Error = Response;

            #[inline]
            fn extract(cx: &mut Context) -> BoxFuture<'_, Result<Self, Self::Error>> {
                Box::pin(async move {
                    Ok((
                        $(
                            match $T::extract(cx).await {
                                Ok(v) => v,
                                Err(e) => return Err(Into::<Response>::into(e as $T::Error)),
                            },
                        )+
                    ))
                })
            }
        }

        impl<Func, $($T,)+ R> HandlerBase<($($T,)+)> for Func
        where
            Func: Fn($($T,)+) -> R + Clone + 'static,
            R: Future + Send + 'static,
            R::Output: Into<Response>,
        {
            type Output = R::Output;
            type Future = R;

            #[inline]
            fn call(&self, args: ($($T,)+)) -> R {
                #[allow(non_snake_case)]
                let ($($T,)+) = args;
                (self)($($T,)+)
            }
        }

        impl<'h, Func, $($T,)+ R> HandlerCamp<'h, ($($T,)+)> for Func
        where
            Func: Fn(&'h mut Context, $($T,)+) -> R + Clone + 'static,
            R: Future + Send + 'h,
            R::Output: Into<Response>,
        {
            type Output = R::Output;
            type Future = R;

            #[inline]
            fn call(&'h self, cx: &'h mut Context, args: ($($T,)+)) -> R {
                #[allow(non_snake_case)]
                let ($($T,)+) = args;
                (self)(cx, $($T,)+)
            }
        }

        peel! { $($T,)+ }
    )
}

tuple! { A, B, C, D, E, F, G, H, I, J, K, L, }
