module Main where

import Prelude ((+), (-), ($), (<>), bind, const, discard, pure, show, unit, Unit)

import Control.Monad.Aff (Canceler, launchAff)
import Control.Monad.Eff (Eff)
import Control.Monad.Eff.Class (liftEff)
import Control.Monad.Eff.Console (log)
import Control.Monad.Eff.Exception (EXCEPTION)

import Data.Foreign (Foreign)
import Data.Maybe (Maybe(Just))

import Graphics.Canvas (CANVAS, getCanvasElementById, getContext2D, setFillStyle, fillPath, rect)

import Network.HTTP.Affjax (affjax, Affjax, AJAX, defaultRequest, get, post)
import Network.HTTP.Affjax.Response (class Respondable)

import Partial.Unsafe (unsafePartial)

import Pux (CoreEffects, EffModel, start)
import Pux.DOM.Events (onClick)
import Pux.DOM.HTML (HTML)
import Pux.Renderer.React (renderToDOM)

import Text.Smolder.HTML (br, button, div, span, h1)
import Text.Smolder.Markup (text, (#!))

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

getToken = launchAff $ do
  res <- get "/connect"
  liftEff $ continueBooting $ "" <> res.response

postPing token = launchAff $ do
  res <- post "/ping" token
  liftEff $ log $ "POST /ping response: " <> res.response

continueBooting token = do
  _ <- postPing token

  launchPux token
  _ <- initCanvas
  pure unit

launchPux initialState = do
  app <- start
    { initialState
    , view
    , foldp
    , inputs: []
    }

  renderToDOM "#app" app.markup app.input

initCanvas = unsafePartial do
  Just canvas <- getCanvasElementById "canvas"
  ctx         <- getContext2D canvas
  _           <- setFillStyle "#0000FF" ctx
  fillPath ctx $ rect ctx { x: 250.0, y: 250.0, w: 100.0, h: 100.0 }

main = getToken
