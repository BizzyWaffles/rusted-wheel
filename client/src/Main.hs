{-# LANGUAGE NoImplicitPrelude #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE RecordWildCards #-}
{-# LANGUAGE ScopedTypeVariables #-}
{-# LANGUAGE TupleSections #-}
{-# LANGUAGE TypeFamilies #-}
module Main where

import Bizzlelude

import Control.Monad(void)
import Control.Monad.Fix(MonadFix)

import GHCJS.DOM(currentDocument, currentWindow)
import GHCJS.DOM.Document(getBody, getCookie)
import GHCJS.DOM.NonElementParentNode(getElementById)
import GHCJS.DOM.Types(Document(Document))
import GHCJS.Marshal(fromJSVal, toJSVal)

import JavaScript.Web.Canvas(arc, beginPath, create, getContext, stroke)
import JavaScript.Web.Canvas.Internal(Canvas(Canvas)) -- Whoopsies

import Reflex.Dom(button, DomBuilder, DomBuilderSpace, el, elAttr, Event, GhcjsDomSpace, mainWidget, MonadHold, PostBuild, text)

import GameState(GameState(competitors, GameState, goons, hourOfDay, news, player), Money(Money), Player(AnonPlayer, anonState, Player), PlayerState(inventory, loadsAMoney, PlayerState, runningTasks, transactions))
import Shim(decodeURIComponent)

import qualified JavaScript.Web.WebSocket as WS
import qualified Prelude                  as GHCJSP
import qualified Data.List                as List
import qualified Data.Map                 as Map
import qualified Data.Set                 as Set
import qualified Data.Text                as Text

data BizzyState =
  BizzyState {
    gameState  :: GameState
  , sendServer :: String -> [String] -> IO ()
  }

main :: IO ()
main =
  do
    socket <- handleSocket
    handleCookie
    let bizzyState = makeInitialState $ \s ss -> return ()
    mainWidget (view $ gameState bizzyState)
    handleCanvas

handleSocket :: IO WS.WebSocket
handleSocket =
  do
    socket <- WS.connect $ WS.WebSocketRequest "ws://localhost:3003/" [] (reactWith "close") (reactWith "message")
    WS.send "It's happening!" socket
    return socket
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

view :: ( DomBuilder t m
        , DomBuilderSpace m ~ GhcjsDomSpace
        , MonadFix m
        , MonadHold t m
        , PostBuild t m
        ) => GameState -> m ()
view (GameState{..}) =
  div $ do
    button "Hired Hands"            -- #! onClick (const NoOp)
    button "Change displayed items" -- #! onClick (const NoOp)
    button "Open shop for the day"  -- #! onClick (const NoOp)
    div $ display "Player inventory"     (inventory    pState)
    div $ display "Player tasks"         (runningTasks pState)
    div $ display "Player money"         (loadsAMoney  pState)
    div $ display "Player transactions"  (transactions pState)
    div $ display "Goons"                goons
    div $ display "Competitors"          competitors
    div $ display "Time of day"          hourOfDay
    div $ display "News"                 news
    elAttr "canvas" (Map.fromList [("height", "700"), ("width", "700"), ("id", "canvas"), ("class", "game-area")]) (return ())
  where
    div           = el "div"
    display label = (showText >>> (\x -> label <> ": " <> x) >>> text)
    pState        = case player of (AnonPlayer anonState) -> anonState
                                   (Player _ _ state)     -> state

makeInitialState :: (String -> [String] -> IO ()) -> BizzyState
makeInitialState sendServer = BizzyState gameState sendServer
  where
    person    = AnonPlayer { anonState = PlayerState { inventory = Set.empty, runningTasks = Set.empty, loadsAMoney = Money 0, transactions = [] } }
    gameState = GameState { player = person, goons = Set.empty, competitors = Set.empty, hourOfDay = 0, news = [] }
