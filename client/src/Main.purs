module Main where

import Prelude

import Control.Monad.Aff (Canceler, launchAff)
import Control.Monad.Eff (Eff)
import Control.Monad.Eff.Class (liftEff)
import Control.Monad.Eff.Console (CONSOLE, log)
import Control.Monad.Eff.Exception (EXCEPTION)

import Data.Either (either, Either(Left))
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

import WebSocket (WEBSOCKET)

import GameState(AnonPlayer(AnonPlayer), GameState(GameState), ID(ID), Money(Money), Player(Player), PlayerState(PlayerState))
import Websocket as WS

data Event = Increment | Decrement | NoOp

newtype BizzyState =
  BizzyState {
    gameState  :: GameState
  , sendServer :: String -> Array String -> (Eff (ws :: WEBSOCKET, err :: EXCEPTION) Unit)
  }

foldp :: forall fx. Event -> BizzyState -> EffModel BizzyState Event fx
foldp Increment (BizzyState bs) = { state: BizzyState bs { gameState = GameState gState { hourOfDay = gState.hourOfDay + 1 } }, effects: [] }
  where { gameState: (GameState gState), sendServer } = bs
foldp Decrement (BizzyState bs) = { state: BizzyState bs { gameState = GameState gState { hourOfDay = gState.hourOfDay - 1 } }, effects: [] }
  where { gameState: (GameState gState), sendServer } = bs
foldp NoOp      bs              = { state: bs                                                                                 , effects: [] }

view :: GameState -> HTML Event
view (GameState { player, goons, competitors, hourOfDay, news }) =
  div do
    button #! onClick (const NoOp) $ text "Hired Hands"
    button #! onClick (const NoOp) $ text "Change displayed items"
    button #! onClick (const NoOp) $ text "Open shop for the day"
    div $ text $ "Player inventory: "    <> show inventory
    div $ text $ "Player's tasks: "      <> show runningTasks
    div $ text $ "Player money: "        <> show money
    div $ text $ "Player transactions: " <> show transactions
    div $ text $ "Goons "                <> show goons
    div $ text $ "Competitors: "         <> show competitors
    div $ text $ "Time of day: "         <> show hourOfDay
    div $ text $ "News: "                <> show news
  where
    PlayerState { inventory, runningTasks, loadsAMoney: (Money money), transactions } = either (\(AnonPlayer { anonState }) -> anonState) (\(Player { state }) -> state) player

makeInitialState :: (String -> Array String -> Eff _ Unit) -> BizzyState
makeInitialState sendServer = BizzyState { gameState, sendServer }
  where
    person    = Left $ AnonPlayer { anonState: PlayerState { inventory: Set.empty, runningTasks: Set.empty, loadsAMoney: Money 0, transactions: [] } }
    gameState = GameState { player: person, goons: Set.empty, competitors: Set.empty, hourOfDay: 0, news: [] }

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
    WS.init host onConnect
  where
    onConnect send =
      do
        app <- start
          { initialState: makeInitialState send
          , view: (\(BizzyState { gameState }) -> gameState) >>> view
          , foldp
          , inputs: []
          }
        renderToDOM "#app" app.markup app.input
        _ <- initCanvas
        pure unit
