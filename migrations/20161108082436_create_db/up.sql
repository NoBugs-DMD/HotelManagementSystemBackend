CREATE TABLE Person (
  ID        SERIAL NOT NULL PRIMARY KEY, 
  Login     varchar(255) NOT NULL UNIQUE, 
  Email     varchar(255) NOT NULL UNIQUE, 
  PassHash  varchar(64) NOT NULL 
);

CREATE TABLE Owner (
  PersonID int4 NOT NULL PRIMARY KEY references Person(ID)
);

CREATE TABLE Manager (
  PersonID int4 NOT NULL PRIMARY KEY references Person(ID)
);

CREATE TABLE Receptionist (
  PersonID int4 NOT NULL PRIMARY KEY references Person(ID)
);

CREATE TABLE Cleaner (
  PersonID int4 NOT NULL PRIMARY KEY references Person(ID)
);

CREATE TABLE Client (
  PersonID           int4 NOT NULL PRIMARY KEY references Person(ID), 
  ClientLevelLevelID int4 NOT NULL
);

CREATE TABLE RuleSet (
  ID              SERIAL NOT NULL PRIMARY KEY, 
  ManagerPersonID int4 NOT NULL references Manager(PersonID), 
  Name            varchar(255) NOT NULL, 
  Body            varchar(255) NOT NULL
);

CREATE TABLE RoomLevel (
  ID        SERIAL NOT NULL PRIMARY KEY, 
  RuleSetID int4 NOT NULL references RuleSet(ID), 
  LevelName varchar(10) NOT NULL, 
  PerNight  int4 
);

CREATE TABLE ClientLevel (
  ID                 SERIAL NOT NULL PRIMARY KEY, 
  RuleSetID          int4 NOT NULL references RuleSet(ID), 
  DiscountPercentage int4 NOT NULL, 
  LevelName          varchar(255) NOT NULL 
);

CREATE TABLE Review (
  ID                SERIAL NOT NULL PRIMARY KEY, 
  Body              int4, 
  LocationRate      int4 NOT NULL, 
  CleanlinessRate   int4 NOT NULL, 
  ServiceRate       int4 NOT NULL, 
  ValueForMoneyRate int4 NOT NULL, 
  CreatedAt         timestamp NOT NULL
);

CREATE TABLE PhotoSet (
  ID SERIAL NOT NULL PRIMARY KEY
);

CREATE TABLE Photo (
  ID   SERIAL NOT NULL PRIMARY KEY, 
  Blob bytea NOT NULL
);

CREATE TABLE PhotoSetPhotos (
  PhotoSetID int4 NOT NULL references PhotoSet(ID), 
  PhotoID    int4 NOT NULL references Photo(ID),
  PRIMARY KEY (PhotoSetID, PhotoID)
);

CREATE TABLE City (
  ID   SERIAL NOT NULL PRIMARY KEY, 
  Name varchar(255) NOT NULL 
);

CREATE TABLE Hotel (
  ID            SERIAL NOT NULL PRIMARY KEY, 
  OwnerPersonID int4 NOT NULL references Owner(PersonID), 
  RuleSetID     int4 NOT NULL references RuleSet(ID), 
  CityID        int4 NOT NULL references City(ID), 
  PhotoSetID    int4 NOT NULL references PhotoSet(ID), 
  Name          varchar(32) NOT NULL, 
  Description   varchar(255) NOT NULL, 
  Rating        int4, 
  Stars         int4 NOT NULL, 
  CONSTRAINT UniqueCityName UNIQUE (CityID, Name)
);

CREATE TABLE EmployedIn (
  PersonID int4 NOT NULL references Person(ID), 
  HotelID  int4 NOT NULL references Hotel(ID), 
  PRIMARY KEY (PersonID, HotelID)
);

CREATE TABLE Room (
  HotelID    int4 NOT NULL references Hotel(ID), 
  RoomNumber int4 NOT NULL,  
  RoomLevelID    int4 NOT NULL references RoomLevel(ID), 
  PhotoSetID int4 references PhotoSet(ID),
  PRIMARY KEY (HotelID, RoomNumber)
);

CREATE TABLE Booking (
  ID             SERIAL NOT NULL PRIMARY KEY, 
  ClientPersonID int4 NOT NULL references Client(PersonID), 
  HotelID        int4 NOT NULL references Hotel(ID), 
  ReviewID       int4 references Review(ID), 
  RoomNumber     int4 NOT NULL, 
  BookingTime    timestamp NOT NULL, 
  ArrivalTime    timestamp NOT NULL, 
  DepartureTime  timestamp NOT NULL, 
  FullCost       int4 NOT NULL, 
  Paid           bool NOT NULL, 
  Cancelled      bool NOT NULL
);

CREATE TABLE MaintainedBy (
  BookingID            int4 NOT NULL references Booking(ID), 
  ReceptionistPersonID int4 NOT NULL references Receptionist(PersonID), 
  MaintainedAt         timestamp NOT NULL, 
  PRIMARY KEY (BookingID, ReceptionistPersonID)
);

CREATE TABLE ToClean (
  ID         SERIAL NOT NULL PRIMARY KEY, 
  HotelID    int4 NOT NULL, 
  RoomNumber int4 NOT NULL, 
  DueTime    timestamp NOT NULL, 
  Done       bool NOT NULL, 
  DoneTime   timestamp NOT NULL, 
  Cancelled  bool NOT NULL,
  FOREIGN KEY (HotelID, RoomNumber) REFERENCES Room (HotelID, RoomNumber)
);

CREATE TABLE AssignedCleaning (
  ToCleanID       int4 NOT NULL references ToClean(ID), 
  CleanerPersonID int4 NOT NULL references Cleaner(PersonID),
  PRIMARY KEY (ToCleanID, CleanerPersonID)
);