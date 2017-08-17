module Main where

import Prelude

import Control.Monad.Aff (Canceler, launchAff)
import Control.Monad.Eff (Eff)
import Control.Monad.Eff.Class (liftEff)
import Control.Monad.Eff.Console (CONSOLE, log)
import Control.Monad.Eff.Exception (EXCEPTION)

import Data.Maybe (Maybe(Just))
import Data.Set as Set

import DOM.HTML(window)
import DOM.HTML.Location(hostname)
import DOM.HTML.Window(location)

import Graphics.Canvas (CANVAS, Context2D, getCanvasElementById, getContext2D, setFillStyle, fillPath, rect)

import Network.HTTP.Affjax (Affjax, AJAX, get, post)

import Partial.Unsafe (unsafePartial)

import Pux (CoreEffects, EffModel, start)
import Pux.DOM.Events (onClick)
import Pux.DOM.HTML (HTML)
import Pux.Renderer.React (renderToDOM)

import Text.Smolder.HTML (button, div)
import Text.Smolder.Markup (text, (#!))

import GameState(GameState(GameState), ID(ID), Money(Money), Player(Player))
import Websocket as WS

data Event = Increment | Decrement | NoOp

foldp :: forall fx. Event -> GameState -> EffModel GameState Event fx
foldp Increment (GameState gameState) = { state: GameState $ gameState { hourOfDay = gameState.hourOfDay + 1 }, effects: [] }
foldp Decrement (GameState gameState) = { state: GameState $ gameState { hourOfDay = gameState.hourOfDay - 1 }, effects: [] }
foldp NoOp                         gs = { state: gs                                                           , effects: [] }

view :: GameState -> HTML Event
view (GameState { player: Player { id: (ID identifier), inventory, name, runningTasks, loadsAMoney: (Money money), transactions }, goons, competitors, hourOfDay, news }) =
  div do
    button #! onClick (const NoOp) $ text "Hired Hands"
    button #! onClick (const NoOp) $ text "Change displayed items"
    button #! onClick (const NoOp) $ text "Open shop for the day"
    div $ text $ "Player ID: "           <> show identifier
    div $ text $ "Player inventory: "    <> show inventory
    div $ text $ "Player name: "         <> name
    div $ text $ "Player's tasks: "      <> show runningTasks
    div $ text $ "Player money: "        <> show money
    div $ text $ "Player transactions: " <> show transactions
    div $ text $ "Goons "                <> show goons
    div $ text $ "Competitors: "         <> show competitors
    div $ text $ "Time of day: "         <> show hourOfDay
    div $ text $ "News: "                <> show news

makeInitialState :: GameState
makeInitialState = GameState { player: person, goons: Set.empty, competitors: Set.empty, hourOfDay: 0, news: [] }
  where
    person = Player { id: ID 9001, inventory: Set.empty, name: "doofus", runningTasks: Set.empty, loadsAMoney: Money 0, transactions: [] }

initCanvas :: forall eff. Eff (canvas :: CANVAS | eff) Context2D
initCanvas = unsafePartial do
  Just canvas <- getCanvasElementById "canvas"
  ctx         <- getContext2D canvas
  _           <- setFillStyle "#0000FF" ctx
  fillPath ctx $ rect ctx { x: 250.0, y: 250.0, w: 100.0, h: 100.0 }

main :: Eff _ Unit
main =
  do
    host <- window >>= location >>= hostname
    WS.init host
    app <- start
      { initialState: makeInitialState
      , view
      , foldp
      , inputs: []
      }
    renderToDOM "#app" app.markup app.input
    _ <- initCanvas
    pure unit
