package sdk.elastic.stream.api;

import sdk.elastic.stream.utils.Arguments;

public class OpenStreamOptions {
    private WriteMode writeMode = WriteMode.SINGLE;
    private ReadMode readMode = ReadMode.MULTIPLE;

    public static Builder newBuilder() {
        return new Builder();
    }

    public WriteMode writeMode() {
        return writeMode;
    }

    public ReadMode readMode() {
        return readMode;
    }

    public enum WriteMode {
        SINGLE(0), MULTIPLE(1);

        final int code;

        WriteMode(int code) {
            this.code = code;
        }

        public int getCode() {
            return code;
        }
    }

    public enum ReadMode {
        SINGLE(0), MULTIPLE(1);

        final int code;

        ReadMode(int code) {
            this.code = code;
        }

        public int getCode() {
            return code;
        }
    }

    public static class Builder {
        private final OpenStreamOptions options = new OpenStreamOptions();

        public Builder writeMode(WriteMode writeMode) {
            Arguments.isNotNull(writeMode, "WriteMode should be set with SINGLE or MULTIPLE");
            options.writeMode = writeMode;
            return this;
        }

        public Builder readMode(ReadMode readMode) {
            Arguments.isNotNull(readMode, "ReadMode should be set with SINGLE or MULTIPLE");
            options.readMode = readMode;
            return this;
        }

        public OpenStreamOptions build() {
            return options;
        }
    }
}
