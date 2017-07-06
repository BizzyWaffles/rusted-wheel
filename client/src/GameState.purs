module GameState(
  AttributeSet
, EntityKind(GoonEntity, NPCEntity, PlayerEntity)
, GameState
, Goon
, ID(ID)
, Item(..)
, Money(Money)
, NewsItem
, Player
, Rating(Rating)
, SkillSet
, Task
, TaskKind(Counterespionage, Scavenge, Spy, Steal)
, Transaction
) where

import Data.Maybe(Maybe)
import Data.Set(Set)
import Data.Tuple(Tuple)

newtype ID = ID Int

newtype Money = Money Int

newtype Rating = Rating Int

data Item
  = Jewelry
  | Berries
  | Potatoes
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

type Player =
  {
    id           :: ID
  , inventory    :: Set Item
  , name         :: String
  , runningTasks :: Set Task
  , loadsAMoney  :: Money
  , token        :: String
  , transactions :: Array Transaction
  }

type Transaction =
  {
    to          :: Tuple ID EntityKind
  , from        :: Tuple ID EntityKind
  , amount      :: Money
  , description :: String
  }

type NewsItem =
  {
    id          :: ID
  , description :: String
  }

type Task =
  {
    id                   :: ID
  , taskType             :: TaskKind
  , effortRequired       :: Number
  , effortPutIn          :: Number
  , currentEffortPerTick :: Number
  }

type Goon =
  {
    id         :: ID
  , name       :: String
  , task       :: Maybe Task
  , attributes :: AttributeSet
  , skills     :: SkillSet
  }

type AttributeSet =
  {
    strength   :: Rating
  , agility    :: Rating
  , charisma   :: Rating
  , perception :: Rating
  }

type SkillSet =
  {
    counterespionage :: Rating
  , scavenge         :: Rating
  , spy              :: Rating
  , steal            :: Rating
  , trade            :: Rating
  }

type GameState =
  {
    player       :: Player
  , goons        :: Set Goon
  , competitors  :: Set Player
  , hourOfDay    :: Int
  , news         :: Array NewsItem
  }
