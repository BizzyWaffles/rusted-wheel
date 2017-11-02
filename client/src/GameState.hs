module GameState(
  AttributeSet(..)
, EntityKind(..)
, GameState(..)
, Goon(..)
, ID(..)
, Item(..)
, Money(..)
, NewsItem(..)
, Player(..)
, PlayerState(..)
, Rating(..)
, SkillSet(..)
, Task(..)
, TaskKind(..)
, Transaction(..)
) where

import Bizzlelude

import Data.Maybe(Maybe)
import Data.Set(Set)

newtype ID = ID Int deriving (Show)

newtype Money = Money Int deriving (Show)

newtype Rating = Rating Int deriving (Show)

data Item
  = Ring
  | Berry
  | Potato
  | Rosemary
  | Stick
  | StrangeFungus
  | TreeSap
  deriving (Show)

data EntityKind
  = GoonEntity
  | NPCEntity
  | PlayerEntity
  deriving (Show)

data TaskKind
  = Counterespionage
  | Scavenge
  | Spy
  | Steal
  | Trade
  deriving (Show)

data PlayerState
  = PlayerState {
      inventory    :: Set Item
    , runningTasks :: Set Task
    , loadsAMoney  :: Money
    , transactions :: [Transaction]
    } deriving (Show)

data Player
  = Player {
      pID    :: ID
    , pName  :: Text
    , pState :: PlayerState
    }
  | AnonPlayer {
      anonState :: PlayerState
    }
  deriving (Show)

data Transaction
  = Transaction {
      to           :: (ID, EntityKind)
    , from         :: (ID, EntityKind)
    , amount       :: Money
    , tDescription :: Text
    } deriving (Show)

data NewsItem
  = NewsItem {
      niID          :: ID
    , niDescription :: Text
    } deriving (Show)

data Task
  = Task {
      taskID               :: ID
    , taskType             :: TaskKind
    , effortRequired       :: Double
    , effortPutIn          :: Double
    , currentEffortPerTick :: Double
    }
  deriving (Show)

data Goon
  = Goon {
      goonID     :: ID
    , goonName   :: Text
    , task       :: Maybe Task
    , attributes :: AttributeSet
    , skills     :: SkillSet
    }
  deriving (Show)

data AttributeSet
  = AttributeSet {
      strength   :: Rating
    , agility    :: Rating
    , charisma   :: Rating
    , perception :: Rating
    }
  deriving (Show)

data SkillSet
  = SkillSet {
      counterespionage :: Rating
    , scavenge         :: Rating
    , spy              :: Rating
    , steal            :: Rating
    , trade            :: Rating
    }
  deriving (Show)

data GameState
  = GameState {
      player       :: Player
    , goons        :: Set Goon
    , competitors  :: Set Player
    , hourOfDay    :: Int
    , news         :: [NewsItem]
    }
