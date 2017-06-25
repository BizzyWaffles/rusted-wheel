module Websocket(init) where

import Prelude

import Control.Monad.Eff (Eff)
import Control.Monad.Eff.Console (log)
import Control.Monad.Eff.Var (($=))

import WebSocket (Connection(Connection), Message(Message), URL(URL), runMessageEvent, runMessage, newWebSocket)

init :: Eff _ Unit
init = do

  Connection socket <- newWebSocket (URL "ws://localhost:2222") []

  socket.onopen $= \event -> do
    socket.send $ Message "hello"
    socket.send $ Message "goodbye"

  socket.onmessage $= \event -> do
    let message = runMessage $ runMessageEvent event
    when (message == "goodbye") do
      socket.close

  socket.onclose $= \event -> do
    log "onclose: Connection closed"
