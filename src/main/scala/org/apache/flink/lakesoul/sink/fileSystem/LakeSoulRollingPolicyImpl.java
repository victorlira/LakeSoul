package org.apache.flink.lakesoul.sink.fileSystem;


import org.apache.flink.lakesoul.tools.LakeSoulKeyGen;
import org.apache.flink.streaming.api.functions.sink.filesystem.PartFileInfo;
import org.apache.flink.table.data.RowData;
import java.io.IOException;



public class LakeSoulRollingPolicyImpl<IN,BucketID> implements LakeSoulRollingPolicy<RowData, String> {

    private  boolean rollOnCheckpoint;

    private LakeSoulKeyGen keygen;

    private long rollingSize =2000L;

    private long rollingTime=1000000000000000L;

    public LakeSoulKeyGen getKeygen() {
        return keygen;
    }

    public void setKeygen(LakeSoulKeyGen keygen) {
        this.keygen = keygen;
    }

    public LakeSoulRollingPolicyImpl(boolean rollOnCheckpoint) {
        this.rollOnCheckpoint = rollOnCheckpoint;
    }

    @Override
    public boolean shouldRollOnCheckpoint(PartFileInfo<String> partFileState) {
        return this.rollOnCheckpoint;
    }

    @Override
    public boolean shouldRollOnEvent(PartFileInfo<String> partFileState, RowData element)
            throws IOException {
        return false;
    }

    @Override
    public boolean shouldRollOnProcessingTime(
            PartFileInfo<String> partFileState, long currentTime) {
        //TODO set time
        return currentTime - partFileState.getLastUpdateTime() > rollingTime;
    }

    public boolean shouldRollOnMaxSize(long size){
        return size > rollingSize;
    }

    @Override
    public boolean shouldRoll(PartFileInfo<String> partFileState, long currentTime ) throws IOException {
        return false;
    }

    public long getRollingSize() {
        return rollingSize;
    }

    public void setRollingSize(long rollingSize) {
        this.rollingSize = rollingSize;
    }
}
