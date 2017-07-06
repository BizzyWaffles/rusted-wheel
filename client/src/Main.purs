module Main where

import Prelude

import Control.Monad.Aff (Canceler, launchAff)
import Control.Monad.Eff (Eff)
import Control.Monad.Eff.Class (liftEff)
import Control.Monad.Eff.Console (CONSOLE, log)
import Control.Monad.Eff.Exception (EXCEPTION)

import Data.Maybe (Maybe(Just))

import Graphics.Canvas (CANVAS, Context2D, getCanvasElementById, getContext2D, setFillStyle, fillPath, rect)

import Network.HTTP.Affjax (Affjax, AJAX, get, post)

import Partial.Unsafe (unsafePartial)

import Pux (CoreEffects, EffModel, start)
import Pux.DOM.Events (onClick)
import Pux.DOM.HTML (HTML)
import Pux.Renderer.React (renderToDOM)

import Text.Smolder.HTML (button, div, h1)
import Text.Smolder.Markup (text, (#!))

import Websocket as WS

data Event = Increment | Decrement | NoOp

type State = String

foldp :: forall fx. Event -> State -> EffModel State Event fx
foldp Increment n = { state: n <> "!", effects: [] }
foldp Decrement n = { state: n <> "?", effects: [] }
foldp NoOp      n = { state: n       , effects: [] }

view :: State -> HTML Event
view token =
  div do
    button #! onClick (const NoOp) $ text "Hired Hands"
    button #! onClick (const NoOp) $ text "Change displayed items"
    button #! onClick (const NoOp) $ text "Open shop for the day"
    h1 $ text token

getToken :: Eff _ (Canceler _)
getToken = launchAff $ do
  res <- get "/connect"
  liftEff $ continueBooting $ "" <> res.response

postPing :: String -> Eff _ (Canceler _)
postPing token = launchAff $ do
  res <- post "/ping" token
  liftEff $ log $ "POST /ping response: " <> res.response

continueBooting :: forall eff. String -> Eff _ Unit
continueBooting token = do
  WS.init "10.105.144.17"
  _ <- postPing token
  launchPux token
  _ <- initCanvas
  pure unit

launchPux :: forall eff. State -> Eff (CoreEffects eff) Unit
launchPux initialState = do
  app <- start
    { initialState
    , view
    , foldp
    , inputs: []
    }
  renderToDOM "#app" app.markup app.input

initCanvas :: forall eff. Eff (canvas :: CANVAS | eff) Context2D
initCanvas = unsafePartial do
  Just canvas <- getCanvasElementById "canvas"
  ctx         <- getContext2D canvas
  _           <- setFillStyle "#0000FF" ctx
  fillPath ctx $ rect ctx { x: 250.0, y: 250.0, w: 100.0, h: 100.0 }

main :: Eff _ (Canceler _)
main = getToken
