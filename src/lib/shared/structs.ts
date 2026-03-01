export type ActivityEntry = {
    startTime: Date;
    endTime: Date;
    title: string;
    processPath: string;
    appName: string;
    isIdle: boolean;
    isAudioPlaying : boolean;
};