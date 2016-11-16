INSERT INTO Person(Login, Name, Email, PassHash) VALUES('owner', 'Owner', 'owner@dmd.ru', '0');
INSERT INTO Person(Login, Name, Email, PassHash) VALUES('manager', 'Manager', 'manager@dmd.ru', '0');
INSERT INTO Person(Login, Name, Email, PassHash) VALUES('receptionist', 'Receptionist', 'receptionist@dmd.ru', '0');
INSERT INTO Person(Login, Name, Email, PassHash) VALUES('cleaner', 'Cleaner', 'cleaner@dmd.ru', '0');

INSERT INTO Owner(PersonID) VALUES ((SELECT Person.ID FROM Person WHERE Person.Login = 'owner'));
INSERT INTO Manager(PersonID) VALUES ((SELECT Person.ID FROM Person WHERE Person.Login = 'manager'));
INSERT INTO Receptionist(PersonID) VALUES ((SELECT Person.ID FROM Person WHERE Person.Login = 'receptionist'));
INSERT INTO Cleaner(PersonID) VALUES ((SELECT Person.ID FROM Person WHERE Person.Login = 'cleaner'));