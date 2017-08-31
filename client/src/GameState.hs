module GameState(
  AttributeSet()
, EntityKind()
, GameState()
, Goon()
, ID()
, Item()
, Money()
, NewsItem()
, Player()
, PlayerState()
, Rating()
, SkillSet()
, Task()
, TaskKind()
, Transaction()
) where

import Bizzlelude

import Data.Maybe(Maybe)
import Data.Set(Set)

newtype ID = ID Int

newtype Money = Money Int

newtype Rating = Rating Int

data Item
  = Ring
  | Berry
  | Potato
  | Rosemary
  | Stick
  | StrangeFungus
  | TreeSap

data EntityKind
  = GoonEntity
  | NPCEntity
  | PlayerEntity

data TaskKind
  = Counterespionage
  | Scavenge
  | Spy
  | Steal
  | Trade

data PlayerState
  = PlayerState {
      inventory    :: Set Item
    , runningTasks :: Set Task
    , loadsAMoney  :: Money
    , transactions :: [Transaction]
    } deriving (Show)

data Player
  = Player {
      id    :: ID
    , name  :: Text
    , state :: PlayerState
    }
  | AnonPlayer {
      anonState :: PlayerState
    }

data AnonPlayer
  = AnonPlayer {
      anonState :: PlayerState
    }

data Transaction
  = Transaction {
      to          :: (ID, EntityKind)
    , from        :: (ID, EntityKind)
    , amount      :: Money
    , description :: Text
    } deriving (Show)

data NewsItem
  = NewsItem {
      id          :: ID
    , description :: Text
    } deriving (Show)

data Task =
  = Task {
      id                   :: ID
    , taskType             :: TaskKind
    , effortRequired       :: Double
    , effortPutIn          :: Double
    , currentEffortPerTick :: Double
    }

data Goon
  = Goon {
      id         :: ID
    , name       :: Text
    , task       :: Maybe Task
    , attributes :: AttributeSet
    , skills     :: SkillSet
    }

data AttributeSet
  = AttributeSet {
      strength   :: Rating
    , agility    :: Rating
    , charisma   :: Rating
    , perception :: Rating
    }

data SkillSet
  = SkillSet {
      counterespionage :: Rating
    , scavenge         :: Rating
    , spy              :: Rating
    , steal            :: Rating
    , trade            :: Rating
    }

data GameState
  = GameState {
      player       :: Player
    , goons        :: Set Goon
    , competitors  :: Set Player
    , hourOfDay    :: Int
    , news         :: [NewsItem]
    }
