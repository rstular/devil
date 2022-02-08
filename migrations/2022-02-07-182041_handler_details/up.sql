ALTER TABLE "handler_events" ADD COLUMN "details" TEXT;
ALTER TABLE "handler_events" ADD COLUMN "x_forwarded_for" TEXT;
ALTER TABLE "handler_events" RENAME COLUMN "info" TO "payload";
