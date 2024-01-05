/*
Copyright 2023 The Matrix.org Foundation C.I.C.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

import "fake-indexeddb/auto";
import {
    BackupKeys,
    BaseMigrationData,
    DeviceId,
    LoggerLevel,
    Migration,
    OlmMachine,
    PickledSession,
    StoreHandle,
    Tracing,
    UserId,
} from "../pkg";

beforeAll(() => {
    new Tracing(LoggerLevel.Trace).turnOn();
});

describe("Migration", () => {
    test("It should correctly import data", async () => {
        const TEST_USER_ID = "@vdhtest200713:matrix.org";
        const TEST_DEVICE_ID = "KMFSTJSMLB";

        const pickleKey = new TextEncoder().encode("+1k2Ppd7HIisUY824v7JtV3/oEE4yX0TqtmNPyhaD7o");

        const store = await StoreHandle.open("testMigration", "testPass");

        const testData = new BaseMigrationData();
        testData.userId = new UserId(TEST_USER_ID);
        testData.deviceId = new DeviceId(TEST_DEVICE_ID);
        testData.pickledAccount =
            "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";
        testData.backupVersion = "3";
        testData.backupRecoveryKey = "/FLbqTHzH1ihmQl3740Dm2aWgOzBng8HjYdGuCpuMLU=";
        testData.privateCrossSigningMasterKey = "oob99xn8lk3eXXERE9U/Zj6gFIsrmAgq3KGvE5Wr0r4=";
        testData.privateCrossSigningSelfSigningKey = "YH1IjbOdpOrIgYZRnQuTInLDV6iSzZ1bNs/UKvUOAII=";
        testData.privateCrossSigningUserSigningKey = "3SFl1AdH3egRKnP5OJZt9wJyamK/SEi8Pfw3dd0mPMo=";
        await Migration.migrateBaseData(testData, pickleKey, store);

        const session1 = new PickledSession();
        session1.pickle =
            "F2tPtegrPKM0c+8Gtw0yyPoQeJn7opKITs/SzFS0QH0uVT8aOTK52/N3p+ATQdWlN2BAsa8MGRXjPPUG+c5s9u/HeZKmpwSiqxgZ9DdbcFYuIy9wiOe4oV68Hu03Yr/vqb9LWPQMTDgSFi2z0u0OMoDCDPB417vztR6fzTE4rwE5HUHgWU1s/7tXcF26nMzeYHuhR8KmpAYgs2/Xt/hcSdsRsyjIVxg4II32gM7XhgYcmQBQewmKasChtmX4V3ihxW6zwib9VwcN+q7XAg01QJyQY4+KSh6YYDSC5j+0on/jhcrpIC4i95i4fFc2Wv5EAVBPB//6TsXsu0s49mkp/H0ZshSeuf/J8Ip9NWI09kl9NM6pNPlalVQQoimFF/FWOovJ8iGQmRpCMmTeJa5CpELZPGXNAPec/eSFqLnSTjyYBFHroaJu9Q";
        session1.senderKey = "1QkuYT/03gzKvMDmKQi5slJvfXECt+ca3/Ue3Cj+Cms";
        session1.lastUseTime = new Date(1703693124932);
        await Migration.migrateOlmSessions([session1], pickleKey, store);

        // now open an olm machine using the store, and check the data.
        const olmMachine = await OlmMachine.initialize(
            new UserId(TEST_USER_ID),
            new DeviceId(TEST_DEVICE_ID),
            "testMigration",
            "testPass",
        );
        expect(olmMachine.identityKeys.curve25519.toBase64()).toEqual("LKv0bKbc0EC4h0jknbemv3QalEkeYvuNeUXVRgVVTTU");
        expect(olmMachine.identityKeys.ed25519.toBase64()).toEqual("qK70DEqIXq7T+UU3v/al47Ab4JkMEBLpNrTBMbS5rrw");

        const backupKeys: BackupKeys = await olmMachine.getBackupKeys();
        expect(backupKeys.backupVersion).toEqual("3");
        expect(backupKeys.decryptionKey?.toBase64()).toEqual("/FLbqTHzH1ihmQl3740Dm2aWgOzBng8HjYdGuCpuMLU");

        // TODO: figure out a way to test cross-signing key import
    }, 15000);
});
