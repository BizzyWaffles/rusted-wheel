data Attribute
  = Strength     -- Damage dealt; how much stock you can move/carry
  | Agility      -- Chance to hit; stealing success; task completion speed
  | Charisma     -- Trading success; spying entry (social engineering)
  | Perception   -- Counter-espionage; spying planning; scavenging

data Skill
  = Scavenge
  | Steal
  | Spy
  | Counterespionage
  | Trade

data Shop
  = Shop { isOpen :: Boolean }

data Item
  = WalkingCane

data Goon
  = Goon { ident :: Long, name :: String, attributes :: [(Attribute, Int)], skills :: [(Skill, Int)], inventory :: [Item] }

data Task
  = Task { ident :: Long, description :: String, assignees :: [Employee], progress :: Number, baseDuration :: Number }

data User
  = User { ident :: Long, shop :: Shop, taskQueue :: [Task] }
