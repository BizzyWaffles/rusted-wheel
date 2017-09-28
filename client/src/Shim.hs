module Shim(decodeURIComponent) where

import Bizzlelude

import Control.Monad.IO.Class(liftIO, MonadIO)

import Data.JSString(JSString, pack, unpack)

import GHCJS.DOM.Types(Window(Window))

foreign import javascript unsafe "$1[\"decodeURIComponent\"]($2)"
  js_decodeURIComponent :: Window -> JSString -> IO JSString

decodeURIComponent :: (MonadIO m) => Window -> Text -> m Text
decodeURIComponent self uri = liftIO $ (unpack >>> asText) <$> (js_decodeURIComponent self $ (show >>> pack) uri)
