-- Taken from https://github.com/dbushenko/purescript-cookies/blob/master/src/Web/Cookies.purs
module Cookie (COOKIE, getCookie) where

import Prelude ((>>>), map, pure)

import Control.Monad.Eff (Eff, kind Effect)

import Data.Array (head)
import Data.Maybe (Maybe(Just, Nothing))

foreign import data COOKIE :: Effect

foreign import _getCookie :: forall eff value. String -> Eff (cookie :: COOKIE | eff) (Array value)

getCookie :: forall eff value. String -> Eff (cookie :: COOKIE | eff) (Maybe value)
getCookie = _getCookie >>> (map head)
