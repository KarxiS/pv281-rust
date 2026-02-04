INSERT INTO "Post" (id, creator_id, created_at, edited_at, deleted_at, content)

VALUES
-- post 1 by smithy.james
('8f1b3c47-410b-4d35-854f-b9b5b00a85b5',
 'fb0f354e-192a-41d2-afc1-0edeee47d316',
 '2023-10-20 13:49:12+01',
 '2023-10-20 13:49:12+01',
 NULL,
 'I had the worst possible day!!! UGH.'),
-- post 2 by smithy.james
('f53457cf-6d93-4de6-bbb3-730f28968d59',
 'fb0f354e-192a-41d2-afc1-0edeee47d316',
 '2023-10-20 15:31:25+01',
 '2023-10-20 15:31:25+01',
 NULL,
 'As if this day could not get any worse... My car has broken'),
-- post 3 by smithy.james
('11474106-ae9f-451d-9aba-b87f581bf498',
 'fb0f354e-192a-41d2-afc1-0edeee47d316',
 '2023-10-20 15:37:01+01',
 '2023-10-20 15:38:20+01',
 NULL,
 'Thankfully, the insurance company will help me...'),
-- post 4 by smithy.jones, deleted
('80d4b1ee-4aa2-4e11-b47e-ccc4483bb662',
 'fb0f354e-192a-41d2-afc1-0edeee47d316',
 '2023-10-20 15:52:44+01',
 '2023-10-20 15:58:30+01',
 '2023-10-20 15:58:30+01',
 'Alright, those scumbags lied to me'),
--- post 5 by dreadfulenergy_
('ad2e9b6b-b26e-43c9-83d0-feaeba739432',
 'b70f9e3d-c4dd-43a4-a319-f3ae3abc3f3d',
 '2023-10-21 20:01:24+01',
 '2023-10-21 20:01:24+01',
 NULL,
 'For the world is cold, and full of vultures.');
