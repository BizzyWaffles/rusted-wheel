module Shim(decodeURIComponent) where

import Bizzlelude

import Control.Monad.IO.Class(liftIO, MonadIO)

import Data.JSString(JSString)

import GHCJS.DOM.Types(fromJSString, FromJSString, toJSString, ToJSString)
import GHCJS.DOM.Window(Window)

foreign import javascript unsafe "$1[\"decodeURIComponent\"]($2)"
  js_decodeURIComponent :: Window -> JSString -> IO JSString

decodeURIComponent :: (MonadIO m, ToJSString uri, FromJSString result) => Window -> uri -> m result
decodeURIComponent self uri = liftIO $ fromJSString <$> (js_decodeURIComponent self $ toJSString uri)
