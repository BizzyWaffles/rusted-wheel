module Main where

import Prelude ((+), (-), ($), bind, const, discard, pure, show, unit, Unit)

import Control.Monad.Eff (Eff)

import Data.Maybe (Maybe(Just))

import Graphics.Canvas (CANVAS, getCanvasElementById, getContext2D, setFillStyle, fillPath, rect)

import Partial.Unsafe (unsafePartial)

import Pux (CoreEffects, EffModel, start)
import Pux.DOM.Events (onClick)
import Pux.DOM.HTML (HTML)
import Pux.Renderer.React (renderToDOM)

import Text.Smolder.HTML (button, div, span)
import Text.Smolder.Markup (text, (#!))

data Event = Increment | Decrement

type State = Int

-- | Return a new state (and effects) from each event
foldp :: forall fx. Event -> State -> EffModel State Event fx
foldp Increment n = { state: n + 1, effects: [] }
foldp Decrement n = { state: n - 1, effects: [] }

-- | Return markup from the state
view :: State -> HTML Event
view count =
  div do
    button #! onClick (const Increment) $ text "Increment"
    span $ text (show count)
    button #! onClick (const Decrement) $ text "Decrement"

-- | Start and render the app
main :: forall fx. Eff (canvas :: CANVAS | CoreEffects fx) Unit
main = unsafePartial do
  app <- start
    { initialState: 0
    , view
    , foldp
    , inputs: []
    }

  renderToDOM "#app" app.markup app.input

  Just canvas <- getCanvasElementById "canvas"
  ctx         <- getContext2D canvas
  _           <- setFillStyle "#0000FF" ctx
  _           <- fillPath ctx $ rect ctx { x: 250.0, y: 250.0, w: 100.0, h: 100.0 }

  pure unit
