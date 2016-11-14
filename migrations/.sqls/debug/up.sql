CREATE TABLE Person (
  ID        SERIAL NOT NULL PRIMARY KEY,
  Name      varchar(255) NOT NULL, 
  Login     varchar(255) NOT NULL UNIQUE, 
  Email     varchar(255) NOT NULL UNIQUE, 
  PassHash  varchar(64) NOT NULL 
);

CREATE TABLE Owner (
  PersonID int4 NOT NULL PRIMARY KEY 
);

CREATE TABLE Manager (
  PersonID int4 NOT NULL PRIMARY KEY 
);

CREATE TABLE Receptionist (
  PersonID int4 NOT NULL PRIMARY KEY 
);

CREATE TABLE Cleaner (
  PersonID int4 NOT NULL PRIMARY KEY 
);

CREATE TABLE Client (
  PersonID      int4 NOT NULL PRIMARY KEY,
  ClientLevelID int4 NOT NULL
);

CREATE TABLE RuleSet (
  ID              SERIAL NOT NULL PRIMARY KEY, 
  ManagerPersonID int4,
  Name            varchar(255) NOT NULL, 
  Body            text NOT NULL
);

CREATE TABLE RoomLevel (
  ID        SERIAL NOT NULL PRIMARY KEY, 
  RuleSetID int4 NOT NULL, 
  LevelName varchar(10) NOT NULL, 
  PerNight  int4 NOT NULL
);

CREATE TABLE ClientLevel (
  ID                 SERIAL NOT NULL PRIMARY KEY, 
  RuleSetID          int4 NOT NULL, 
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
  PhotoSetID int4 NOT NULL,
  PhotoID    int4 NOT NULL,
  PRIMARY KEY (PhotoSetID, PhotoID)
);

CREATE TABLE City (
  ID   SERIAL NOT NULL PRIMARY KEY, 
  Name varchar(255) NOT NULL 
);

CREATE TABLE Hotel (
  ID            SERIAL NOT NULL PRIMARY KEY, 
  OwnerPersonID int4 NOT NULL, 
  RuleSetID     int4 NOT NULL,
  CityID        int4 NOT NULL,
  PhotoSetID    int4,
  Name          varchar(32) NOT NULL, 
  Description   varchar(255) NOT NULL, 
  Rating        int4, 
  Stars         int4 NOT NULL, 
  CONSTRAINT UniqueCityName UNIQUE (CityID, Name)
);

CREATE TABLE EmployedIn (
  PersonID int4 NOT NULL,
  HotelID  int4 NOT NULL,
  PRIMARY KEY (PersonID, HotelID)
);

CREATE TABLE Room (
  HotelID     int4 NOT NULL,
  RoomNumber  int4 NOT NULL,  
  RoomLevelID int4 NOT NULL, 
  PhotoSetID  int4,
  PRIMARY KEY (HotelID, RoomNumber)
);

CREATE TABLE Booking (
  ID             SERIAL NOT NULL PRIMARY KEY, 
  ClientPersonID int4 NOT NULL, 
  HotelID        int4 NOT NULL,
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
  BookingID         int4 NOT NULL,
  Body              text, 
  LocationRate      int4 NOT NULL, 
  CleanlinessRate   int4 NOT NULL, 
  ServiceRate       int4 NOT NULL, 
  ValueForMoneyRate int4 NOT NULL, 
  CreatedAt         timestamp NOT NULL
);

CREATE TABLE MaintainedBy (
  BookingID            int4 NOT NULL, 
  ReceptionistPersonID int4 NOT NULL,
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
  Cancelled  boolean NOT NULL
);

CREATE TABLE AssignedCleaning (
  ToCleanID       int4 NOT NULL,
  CleanerPersonID int4 NOT NULL,
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

-- Insert Booking and update MaintainedBy
CREATE OR REPLACE FUNCTION insert_booking_and_return_id(ClientPersonID int4, 
                                                        HotelID        int4,  
                                                        RoomNumber     int4, 
                                                        BookingTime    timestamp, 
                                                        ArrivalTime    timestamp, 
                                                        DepartureTime  timestamp) 
                                                        RETURNS int4 as $insert_booking_and_return_id$
DECLARE
new_id int4;
BEGIN
    INSERT INTO Booking 
    VALUES(ClientPersonID, HotelID, RoomNumber, BookingTime, ArrivalTime, DepartureTime, 0, false, false) 
    RETURNING id into new_id;
    RETURN new_id;
END;
$insert_booking_and_return_id$
LANGUAGE 'plpgsql';

-- Trigger to auto calculate the cost on Booking insert
CREATE OR REPLACE FUNCTION calculate_booking_cost() RETURNS TRIGGER as $calculate_booking_cost$
DECLARE
BEGIN
    -- Get cost per-night for booked room level
    DROP TABLE IF EXISTS per_night;
    CREATE TEMP table per_night on commit drop
    AS SELECT RL.PerNight FROM RoomLevel as RL, Room as R
    WHERE R.HotelID = new.HotelID and R.RoomNumber = new.RoomNumber and R.RoomLevelID = RL.ID;

    -- Check if per-night cost have not been retrieved
    IF NOT EXISTS (SELECT * FROM per_night) then
      RAISE EXCEPTION 'No Room or LevelID';
    END IF;

    -- Calculate number of nights
    -- Set cost
    new.FullCost := per_night.PerNight * EXTRACT(DAY FROM (new.ArrivalTime, new.DepartureTime)); 

    RETURN new;
END;
$calculate_booking_cost$
LANGUAGE 'plpgsql';

CREATE TRIGGER calculate_booking_cost BEFORE INSERT ON Booking
    FOR EACH ROW EXECUTE PROCEDURE calculate_booking_cost(); 