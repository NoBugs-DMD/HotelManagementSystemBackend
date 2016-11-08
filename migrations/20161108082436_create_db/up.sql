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
  PersonID      int4 NOT NULL PRIMARY KEY references Person(ID), 
  ClientLevelID int4 NOT NULL
);

CREATE TABLE RuleSet (
  ID              SERIAL NOT NULL PRIMARY KEY, 
  ManagerPersonID int4 references Manager(PersonID), 
  Name            varchar(255) NOT NULL, 
  Body            text NOT NULL
);

CREATE TABLE RoomLevel (
  ID        SERIAL NOT NULL PRIMARY KEY, 
  RuleSetID int4 NOT NULL references RuleSet(ID), 
  LevelName varchar(10) NOT NULL, 
  PerNight  int4 NOT NULL
);

CREATE TABLE ClientLevel (
  ID                 SERIAL NOT NULL PRIMARY KEY, 
  RuleSetID          int4 NOT NULL references RuleSet(ID), 
  DiscountPercentage int4 NOT NULL, 
  LevelName          varchar(255) NOT NULL 
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
  HotelID     int4 NOT NULL references Hotel(ID), 
  RoomNumber  int4 NOT NULL,  
  RoomLevelID int4 NOT NULL references RoomLevel(ID), 
  PhotoSetID  int4 references PhotoSet(ID),
  PRIMARY KEY (HotelID, RoomNumber)
);

CREATE TABLE Booking (
  ID             SERIAL NOT NULL PRIMARY KEY, 
  ClientPersonID int4 NOT NULL references Client(PersonID), 
  HotelID        int4 NOT NULL references Hotel(ID),  
  RoomNumber     int4 NOT NULL, 
  BookingTime    timestamp NOT NULL, 
  ArrivalTime    timestamp NOT NULL, 
  DepartureTime  timestamp NOT NULL, 
  FullCost       int4 NOT NULL, 
  Paid           boolean NOT NULL, 
  Cancelled      boolean NOT NULL
);

CREATE TABLE Review (
  ID                SERIAL NOT NULL PRIMARY KEY, 
  BookingID         int4 NOT NULL references Booking(ID),
  Body              text, 
  LocationRate      int4 NOT NULL, 
  CleanlinessRate   int4 NOT NULL, 
  ServiceRate       int4 NOT NULL, 
  ValueForMoneyRate int4 NOT NULL, 
  CreatedAt         timestamp NOT NULL
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
  Done       boolean NOT NULL, 
  DoneTime   timestamp NOT NULL, 
  Cancelled  boolean NOT NULL,
  FOREIGN KEY (HotelID, RoomNumber) REFERENCES Room (HotelID, RoomNumber)
);

CREATE TABLE AssignedCleaning (
  ToCleanID       int4 NOT NULL references ToClean(ID), 
  CleanerPersonID int4 NOT NULL references Cleaner(PersonID),
  PRIMARY KEY (ToCleanID, CleanerPersonID)
);

-- Trigger to auto add registered users to Client table

CREATE OR REPLACE FUNCTION auto_add_client() RETURNS TRIGGER as $emp_auto_add_client$
DECLARE
BEGIN
    INSERT INTO Client (PersonID, ClientLevelID) values (new.ID, 0);
    RETURN new;
END;
$emp_auto_add_client$
LANGUAGE 'plpgsql';

CREATE TRIGGER auto_add_client AFTER INSERT ON Person
    FOR EACH ROW EXECUTE PROCEDURE auto_add_client(); 