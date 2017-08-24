module GameState(
  AttributeSet
, EntityKind(GoonEntity, NPCEntity, PlayerEntity)
, GameState(GameState)
, Goon
, ID(ID)
, Item(..)
, Money(Money)
, NewsItem
, Player(Player)
, AnonPlayer(AnonPlayer)
, PlayerState(PlayerState)
, Rating(Rating)
, SkillSet
, Task
, TaskKind(Counterespionage, Scavenge, Spy, Steal)
, Transaction
) where

import Data.Either(Either)
import Data.Generic.Rep(class Generic)
import Data.Generic.Rep.Show(genericShow)
import Data.Maybe(Maybe)
import Data.Set(Set)
import Data.Show(class Show)
import Data.Tuple(Tuple)

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

newtype PlayerState =
  PlayerState {
    inventory    :: Set Item
  , runningTasks :: Set Task
  , loadsAMoney  :: Money
  , transactions :: Array Transaction
  }

newtype Player =
  Player {
    id    :: ID
  , name  :: String
  , state :: PlayerState
  }

newtype AnonPlayer =
  AnonPlayer {
    anonState :: PlayerState
  }

newtype Transaction =
  Transaction {
    to          :: Tuple ID EntityKind
  , from        :: Tuple ID EntityKind
  , amount      :: Money
  , description :: String
  }

newtype NewsItem =
  NewsItem {
    id          :: ID
  , description :: String
  }

newtype Task =
  Task {
    id                   :: ID
  , taskType             :: TaskKind
  , effortRequired       :: Number
  , effortPutIn          :: Number
  , currentEffortPerTick :: Number
  }

newtype Goon =
  Goon {
    id         :: ID
  , name       :: String
  , task       :: Maybe Task
  , attributes :: AttributeSet
  , skills     :: SkillSet
  }

newtype AttributeSet =
  AttributeSet {
    strength   :: Rating
  , agility    :: Rating
  , charisma   :: Rating
  , perception :: Rating
  }

newtype SkillSet =
  SkillSet {
    counterespionage :: Rating
  , scavenge         :: Rating
  , spy              :: Rating
  , steal            :: Rating
  , trade            :: Rating
  }

newtype GameState =
  GameState {
    player       :: Either AnonPlayer Player
  , goons        :: Set Goon
  , competitors  :: Set Player
  , hourOfDay    :: Int
  , news         :: Array NewsItem
  }

derive instance genericAttributeSet :: Generic AttributeSet _
derive instance genericEntityKind   :: Generic EntityKind   _
derive instance genericGameState    :: Generic GameState    _
derive instance genericGoon         :: Generic Goon         _
derive instance genericID           :: Generic ID           _
derive instance genericItem         :: Generic Item         _
derive instance genericMoney        :: Generic Money        _
derive instance genericNewsItem     :: Generic NewsItem     _
derive instance genericPlayer       :: Generic Player       _
derive instance genericAnonPlayer   :: Generic AnonPlayer   _
derive instance genericPlayerState  :: Generic PlayerState  _
derive instance genericRating       :: Generic Rating       _
derive instance genericSkillSet     :: Generic SkillSet     _
derive instance genericTask         :: Generic Task         _
derive instance genericTaskKind     :: Generic TaskKind     _
derive instance genericTransaction  :: Generic Transaction  _

instance showAttributeSet :: Show AttributeSet where
  show = genericShow

instance showEntityKind :: Show EntityKind where
  show = genericShow

instance showGameState :: Show GameState where
  show = genericShow

instance showGoon :: Show Goon where
  show = genericShow

instance showID :: Show ID where
  show = genericShow

instance showItem :: Show Item where
  show = genericShow

instance showMoney :: Show Money where
  show = genericShow

instance showNewsItem :: Show NewsItem where
  show = genericShow

instance showPlayer :: Show Player where
  show = genericShow

instance showAnonPlayer :: Show AnonPlayer where
  show = genericShow

instance showPlayerState :: Show PlayerState where
  show = genericShow

instance showRating :: Show Rating where
  show = genericShow

instance showSkillSet :: Show SkillSet where
  show = genericShow

instance showTask :: Show Task where
  show = genericShow

instance showTaskKind :: Show TaskKind where
  show = genericShow

instance showTransaction :: Show Transaction where
  show = genericShow
