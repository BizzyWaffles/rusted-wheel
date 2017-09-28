{-# LANGUAGE ScopedTypeVariables #-}
{-# LANGUAGE TupleSections #-}
module Main where

import Bizzlelude

import Control.Monad(void)

import GHCJS.DOM(currentDocument, currentWindow)
import GHCJS.DOM.Document(getBody, getCookie)
import GHCJS.DOM.NonElementParentNode(getElementById)
import GHCJS.DOM.Types(Document(Document))
import GHCJS.Marshal(fromJSVal, toJSVal)

import JavaScript.Web.Canvas(arc, beginPath, create, getContext, stroke)
import JavaScript.Web.Canvas.Internal(Canvas(Canvas)) -- Whoopsies

import Shim(decodeURIComponent)

import qualified JavaScript.Web.WebSocket as WS
import qualified Prelude                  as GHCJSP
import qualified Data.List                as List
import qualified Data.Map                 as Map
import qualified Data.Text                as Text

main :: IO ()
main =
  do
    handleSocket
    handleCookie
    handleCanvas

handleSocket :: IO ()
handleSocket =
  do
    socket <- WS.connect $ WS.WebSocketRequest "ws://localhost:3003/" [] (reactWith "close") (reactWith "message")
    WS.send "It's happening!" socket
  where
    reactWith = asString >>> GHCJSP.print >>> const >>> Just

handleCookie :: IO ()
handleCookie =
  do
    Just document  <- currentDocument
    cookieMap      <- getCookieMap document
    let Just value = Map.lookup "bzwf_anon_wstx" cookieMap
    GHCJSP.print value
  where
    getCookieMap :: Document -> IO (Map Text Text)
    getCookieMap document =
      do
        cookie      <- getCookie document
        Just window <- currentWindow
        cookie |> (asText >>> (Text.split isCookieSep) >>> sanitize >>> (mapM $ parseParts window) >>> (map Map.fromList))
      where
        isCookieSep ','   = True
        isCookieSep ';'   = True
        isCookieSep   _   = False
        sanitize          = (map Text.strip) >>> (List.filter $ Text.any (== '='))
        secondM f (a, b)  = map (a, ) (f b)
        parseParts window = Text.breakOn "=" >>> secondM (Text.tail >>> (decodeURIComponent window))

handleCanvas :: IO ()
handleCanvas =
  do
    Just document <- currentDocument
    Just elem     <- getElementById document ("canvas" :: Text)
    jsValCanvas   <- toJSVal elem
    let canvas = Canvas jsValCanvas
    ctx <- getContext canvas
    beginPath ctx
    arc 95 80 52 0 (2 * pi) False ctx
    stroke ctx
