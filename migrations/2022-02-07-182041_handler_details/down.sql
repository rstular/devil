ALTER TABLE "handler_events" DROP COLUMN "details";
ALTER TABLE "handler_events" DROP COLUMN "x_forwarded_for";
ALTER TABLE "handler_events" RENAME COLUMN "payload" TO "info";