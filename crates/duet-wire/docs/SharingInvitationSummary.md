# SharingInvitationSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  |
**expires_at** | **chrono::DateTime<chrono::FixedOffset>** |  |
**invitation_id** | **uuid::Uuid** |  |
**stat_ids** | **Vec<StatIds>** |  (enum: steps, activeEnergy, exerciseMinutes, sleepMinutes, restingHeartRate, distance, bodyMass, dietaryEnergy, water) |
**state** | **State** |  (enum: pending, accepted, declined, cancelled, expired) |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


