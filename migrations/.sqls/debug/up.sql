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
  Body            text NOT NULL,
  IsDefault       boolean NOT NULL
);

CREATE TABLE RoomLevel (
  Level     int4 NOT NULL, 
  RuleSetID int4 NOT NULL, 
  LevelName varchar(10), 
  PerNight  int4 NOT NULL,
  PRIMARY KEY (Level, RuleSetID)
);

CREATE TABLE ClientLevel (
  BookingsAmount     int4 NOT NULL, 
  RuleSetID          int4 NOT NULL, 
  LevelName          varchar(255),
  DiscountPercentage int4 NOT NULL, 
  PRIMARY KEY (BookingsAmount, RuleSetID)
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
  RoomLevel   int4 NOT NULL, 
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

-- Insert Booking and return ID
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
    INSERT INTO Booking (ClientPersonID, HotelID, RoomNumber, BookingTime, ArrivalTime, DepartureTime, FullCost, Paid, Cancelled)
    VALUES(ClientPersonID, HotelID, RoomNumber, BookingTime, ArrivalTime, DepartureTime, 0, false, false) 
    RETURNING id into new_id;
    RETURN new_id;
END;
$insert_booking_and_return_id$
LANGUAGE 'plpgsql';

-- Insert Hotel and return ID
CREATE OR REPLACE FUNCTION insert_hotel_and_return_id(OwnerPersonID int4,
                                                      RuleSetID int4,
                                                      CityID int4,
                                                      PhotoSetID int4,
                                                      Name varchar(32),
                                                      Description varchar(255),
                                                      Rating int4,
                                                      Stars int4) 
                                                      RETURNS int4 as $insert_hotel_and_return_id$
DECLARE
new_id int4;
BEGIN
    INSERT INTO Hotel (OwnerPersonID, RuleSetID, CityID, PhotoSetID, Name, Description, Rating, Stars) 
    VALUES(OwnerPersonID, RuleSetID, CityID, PhotoSetID, Name, Description, Rating, Stars) 
    RETURNING id into new_id;
    RETURN new_id;
END;
$insert_hotel_and_return_id$
LANGUAGE 'plpgsql';

-- Calculate cost
CREATE OR REPLACE FUNCTION room_cost(room_level int4, hotel_id int4, client_id int4) RETURNS int4 as $room_cost$
DECLARE
per_night int4;
discount int4;
BEGIN
    -- Get cost per-night for booked room level
    SELECT INTO per_night RoomLevel.PerNight FROM RoomLevel, Hotel
    WHERE Hotel.ID = hotel_id and Hotel.RuleSetID = RoomLevel.RuleSetID
      AND RoomLevel.Level = room_level;

    -- Get discount
    SELECT INTO discount min(ClientLevel.DiscountPercentage) FROM ClientLevel, Hotel
    WHERE Hotel.ID = hotel_id and ClientLevel.RuleSetID = Hotel.RuleSetID
      AND ClientLevel.BookingsAmount > (SELECT count(*) FROM Booking 
                                        WHERE Booking.ClientPersonID = client_id);

    RETURN ((100 - discoint)/100) * 
                    per_night.PerNight;
END;
$room_cost$
LANGUAGE 'plpgsql';

-- Trigger to auto calculate the cost on Booking insert
CREATE OR REPLACE FUNCTION calculate_booking_cost() RETURNS TRIGGER as $calculate_booking_cost$
DECLARE
room_level int4;
BEGIN
    SELECT INTO room_level RoomLevel.Level FROM Room, RoomLevel
    WHERE Room.HotelID = new.HotelID and Room.RoomNumber = new.RoomNumber;

    -- Calculate number of nights
    -- Set cost
    new.FullCost := (SELECT room_cost(room_level, new.HotelID, new.ClientID)) * 
                    EXTRACT(DAY FROM (new.ArrivalTime, new.DepartureTime)); 

    RETURN new;
END;
$calculate_booking_cost$
LANGUAGE 'plpgsql';

CREATE TRIGGER calculate_booking_cost BEFORE INSERT ON Booking
    FOR EACH ROW EXECUTE PROCEDURE calculate_booking_cost(); 