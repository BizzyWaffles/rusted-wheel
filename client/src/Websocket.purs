module Websocket(init) where

import Prelude

import Control.Monad.Eff (Eff)
import Control.Monad.Eff.Console (log)
import Control.Monad.Eff.Exception (EXCEPTION)
import Control.Monad.Eff.Var (($=))

import Cookie (getCookie)

import Data.Array((:))
import Data.Maybe (Maybe(Just))
import Data.String (joinWith)

import Partial.Unsafe (unsafePartial)

import WebSocket (Connection(Connection), Message(Message), URL(URL), runMessageEvent, runMessage, newWebSocket, WEBSOCKET)

init :: String -> ((String -> Array String -> (Eff (ws :: WEBSOCKET, err :: EXCEPTION) Unit)) -> Eff _ Unit) -> Eff _ Unit
init ipAddress onConnect =
  do

    Connection socket <- newWebSocket (URL $ "ws://" <> ipAddress <> ":3001") []

    socket.onopen $= \event -> unsafePartial do
      log "onopen: Connection opened"
      Just token <- getCookie "bzwf_anon_wstx"
      onConnect $ \typ args -> socket.send $ Message $ joinWith ":" $ [token, typ] <> args

    socket.onmessage $= \event -> do
      let message = runMessage $ runMessageEvent event
      when (message == "goodbye") do
        socket.close

    socket.onclose $= \event -> do
      log "onclose: Connection closed"
