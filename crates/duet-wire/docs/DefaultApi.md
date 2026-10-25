# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_durable_conversation_command**](DefaultApi.md#accept_durable_conversation_command) | **POST** /api/conversation/{id}/commands |
[**apple_health_source_api_evidence_apple_health_get**](DefaultApi.md#apple_health_source_api_evidence_apple_health_get) | **GET** /api/evidence/apple-health | Apple Health Source
[**commit_durable_privacy_restriction**](DefaultApi.md#commit_durable_privacy_restriction) | **POST** /api/auth/privacy-restrictions |
[**complete_epic_api_connectors_epic_complete_post**](DefaultApi.md#complete_epic_api_connectors_epic_complete_post) | **POST** /api/connectors/epic/complete | Complete Epic
[**complete_healow_api_connectors_healow_complete_post**](DefaultApi.md#complete_healow_api_connectors_healow_complete_post) | **POST** /api/connectors/healow/complete | Complete Healow
[**complete_oracle_api_connectors_oracle_complete_post**](DefaultApi.md#complete_oracle_api_connectors_oracle_complete_post) | **POST** /api/connectors/oracle/complete | Complete Oracle
[**connect_apple_health_api_evidence_apple_health_connect_post**](DefaultApi.md#connect_apple_health_api_evidence_apple_health_connect_post) | **POST** /api/evidence/apple-health/connect | Connect Apple Health
[**connect_health_records_api_evidence_health_records_connect_post**](DefaultApi.md#connect_health_records_api_evidence_health_records_connect_post) | **POST** /api/evidence/health-records/connect | Connect Health Records
[**disconnect_apple_health_api_evidence_apple_health_delete**](DefaultApi.md#disconnect_apple_health_api_evidence_apple_health_delete) | **DELETE** /api/evidence/apple-health | Disconnect Apple Health
[**disconnect_epic_api_connectors_epic_organization_id_delete**](DefaultApi.md#disconnect_epic_api_connectors_epic_organization_id_delete) | **DELETE** /api/connectors/epic/{organization_id} | Disconnect Epic
[**disconnect_healow_api_connectors_healow_organization_id_delete**](DefaultApi.md#disconnect_healow_api_connectors_healow_organization_id_delete) | **DELETE** /api/connectors/healow/{organization_id} | Disconnect Healow
[**disconnect_health_records_api_evidence_health_records_delete**](DefaultApi.md#disconnect_health_records_api_evidence_health_records_delete) | **DELETE** /api/evidence/health-records | Disconnect Health Records
[**disconnect_oracle_api_connectors_oracle_organization_id_delete**](DefaultApi.md#disconnect_oracle_api_connectors_oracle_organization_id_delete) | **DELETE** /api/connectors/oracle/{organization_id} | Disconnect Oracle
[**evidence_lab_api_evidence_labs_post**](DefaultApi.md#evidence_lab_api_evidence_labs_post) | **POST** /api/evidence/labs | Evidence Lab
[**evidence_library_api_evidence_library_post**](DefaultApi.md#evidence_library_api_evidence_library_post) | **POST** /api/evidence/library | Evidence Library
[**evidence_records_api_evidence_records_get**](DefaultApi.md#evidence_records_api_evidence_records_get) | **GET** /api/evidence/records | Evidence Records
[**evidence_series_api_evidence_series_post**](DefaultApi.md#evidence_series_api_evidence_series_post) | **POST** /api/evidence/series | Evidence Series
[**evidence_state_api_evidence_state_get**](DefaultApi.md#evidence_state_api_evidence_state_get) | **GET** /api/evidence/state | Evidence State
[**evidence_timeline_api_evidence_timeline_post**](DefaultApi.md#evidence_timeline_api_evidence_timeline_post) | **POST** /api/evidence/timeline | Evidence Timeline
[**export_approved_doctor_page**](DefaultApi.md#export_approved_doctor_page) | **GET** /api/doctor-collection/pdf |
[**health_health_get**](DefaultApi.md#health_health_get) | **GET** /health | Health
[**health_records_source_api_evidence_health_records_get**](DefaultApi.md#health_records_source_api_evidence_health_records_get) | **GET** /api/evidence/health-records | Health Records Source
[**ingest_canonical_evidence**](DefaultApi.md#ingest_canonical_evidence) | **POST** /api/canonical-evidence |
[**ingest_evidence_api_evidence_post**](DefaultApi.md#ingest_evidence_api_evidence_post) | **POST** /api/evidence | Ingest Evidence
[**portal_organizations_api_connectors_portals_organizations_get**](DefaultApi.md#portal_organizations_api_connectors_portals_organizations_get) | **GET** /api/connectors/portals/organizations | Portal Organizations
[**portal_status_api_connectors_portals_get**](DefaultApi.md#portal_status_api_connectors_portals_get) | **GET** /api/connectors/portals | Portal Status
[**preview_approved_doctor_page**](DefaultApi.md#preview_approved_doctor_page) | **GET** /api/doctor-collection/preview |
[**publish_approved_doctor_page**](DefaultApi.md#publish_approved_doctor_page) | **POST** /api/doctor-collection/share-link |
[**read_canonical_evidence_exact**](DefaultApi.md#read_canonical_evidence_exact) | **POST** /api/canonical-evidence/exact |
[**read_canonical_evidence_snapshot**](DefaultApi.md#read_canonical_evidence_snapshot) | **GET** /api/canonical-evidence |
[**read_canonical_health_context**](DefaultApi.md#read_canonical_health_context) | **GET** /api/health-context |
[**read_doctor_publication**](DefaultApi.md#read_doctor_publication) | **GET** /api/doctor-collection/share-link |
[**read_doctor_save_receipt**](DefaultApi.md#read_doctor_save_receipt) | **GET** /api/doctor-collection/save-receipts/{generation} |
[**read_durable_account_context**](DefaultApi.md#read_durable_account_context) | **GET** /api/auth/account/context |
[**read_durable_conversation**](DefaultApi.md#read_durable_conversation) | **GET** /api/conversation/{id} |
[**read_durable_conversation_events**](DefaultApi.md#read_durable_conversation_events) | **GET** /api/conversation/commands/{id}/events |
[**read_durable_conversation_usage**](DefaultApi.md#read_durable_conversation_usage) | **GET** /api/conversation/commands/{id}/usage |
[**read_today_assessment_status**](DefaultApi.md#read_today_assessment_status) | **POST** /api/insight-cycles/status |
[**reject_durable_doctor_proposal**](DefaultApi.md#reject_durable_doctor_proposal) | **POST** /api/doctor-collection/proposals/{id}/reject |
[**resume_durable_conversation_command**](DefaultApi.md#resume_durable_conversation_command) | **POST** /api/conversation/commands/{id}/resume |
[**retrieve_published_doctor_pdf**](DefaultApi.md#retrieve_published_doctor_pdf) | **GET** /shared/for-my-doctor/{token}/pdf |
[**retrieve_smart_health_link_file**](DefaultApi.md#retrieve_smart_health_link_file) | **GET** /shl-file/{location} |
[**retrieve_smart_health_link_manifest**](DefaultApi.md#retrieve_smart_health_link_manifest) | **POST** /shl/{token}/manifest |
[**revoke_doctor_publication**](DefaultApi.md#revoke_doctor_publication) | **DELETE** /api/doctor-collection/share-link |
[**start_epic_api_connectors_epic_start_post**](DefaultApi.md#start_epic_api_connectors_epic_start_post) | **POST** /api/connectors/epic/start | Start Epic
[**start_healow_api_connectors_healow_start_post**](DefaultApi.md#start_healow_api_connectors_healow_start_post) | **POST** /api/connectors/healow/start | Start Healow
[**start_oracle_api_connectors_oracle_start_post**](DefaultApi.md#start_oracle_api_connectors_oracle_start_post) | **POST** /api/connectors/oracle/start | Start Oracle
[**sync_epic_api_connectors_epic_organization_id_sync_post**](DefaultApi.md#sync_epic_api_connectors_epic_organization_id_sync_post) | **POST** /api/connectors/epic/{organization_id}/sync | Sync Epic
[**sync_healow_api_connectors_healow_organization_id_sync_post**](DefaultApi.md#sync_healow_api_connectors_healow_organization_id_sync_post) | **POST** /api/connectors/healow/{organization_id}/sync | Sync Healow
[**sync_oracle_api_connectors_oracle_organization_id_sync_post**](DefaultApi.md#sync_oracle_api_connectors_oracle_organization_id_sync_post) | **POST** /api/connectors/oracle/{organization_id}/sync | Sync Oracle



## accept_durable_conversation_command

> std::collections::HashMap<String, serde_json::Value> accept_durable_conversation_command(id, accept_durable_conversation_command_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**accept_durable_conversation_command_request** | [**AcceptDurableConversationCommandRequest**](AcceptDurableConversationCommandRequest.md) |  | [required] |

### Return type

**std::collections::HashMap<String, serde_json::Value>**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apple_health_source_api_evidence_apple_health_get

> models::PhoneHealthSourceResponse apple_health_source_api_evidence_apple_health_get(x_duet_source_key)
Apple Health Source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_duet_source_key** | **String** |  | [required] |

### Return type

[**models::PhoneHealthSourceResponse**](PhoneHealthSourceResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## commit_durable_privacy_restriction

> models::DurablePrivacyReceipt commit_durable_privacy_restriction(durable_restriction_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**durable_restriction_request** | [**DurableRestrictionRequest**](DurableRestrictionRequest.md) |  | [required] |

### Return type

[**models::DurablePrivacyReceipt**](DurablePrivacyReceipt.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_epic_api_connectors_epic_complete_post

> complete_epic_api_connectors_epic_complete_post(epic_completion_request)
Complete Epic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_completion_request** | [**EpicCompletionRequest**](EpicCompletionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_healow_api_connectors_healow_complete_post

> complete_healow_api_connectors_healow_complete_post(epic_completion_request)
Complete Healow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_completion_request** | [**EpicCompletionRequest**](EpicCompletionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_oracle_api_connectors_oracle_complete_post

> complete_oracle_api_connectors_oracle_complete_post(epic_completion_request)
Complete Oracle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_completion_request** | [**EpicCompletionRequest**](EpicCompletionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connect_apple_health_api_evidence_apple_health_connect_post

> models::AppleHealthConnectResponse connect_apple_health_api_evidence_apple_health_connect_post(sharing_device_request)
Connect Apple Health

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_device_request** | [**SharingDeviceRequest**](SharingDeviceRequest.md) |  | [required] |

### Return type

[**models::AppleHealthConnectResponse**](AppleHealthConnectResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connect_health_records_api_evidence_health_records_connect_post

> models::HealthRecordsConnectResponse connect_health_records_api_evidence_health_records_connect_post(sharing_device_request)
Connect Health Records

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_device_request** | [**SharingDeviceRequest**](SharingDeviceRequest.md) |  | [required] |

### Return type

[**models::HealthRecordsConnectResponse**](HealthRecordsConnectResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disconnect_apple_health_api_evidence_apple_health_delete

> models::AppleHealthDisconnectResponse disconnect_apple_health_api_evidence_apple_health_delete()
Disconnect Apple Health

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AppleHealthDisconnectResponse**](AppleHealthDisconnectResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disconnect_epic_api_connectors_epic_organization_id_delete

> models::EpicDisconnectResponse disconnect_epic_api_connectors_epic_organization_id_delete(organization_id)
Disconnect Epic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

[**models::EpicDisconnectResponse**](EpicDisconnectResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disconnect_healow_api_connectors_healow_organization_id_delete

> models::EpicDisconnectResponse disconnect_healow_api_connectors_healow_organization_id_delete(organization_id)
Disconnect Healow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

[**models::EpicDisconnectResponse**](EpicDisconnectResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disconnect_health_records_api_evidence_health_records_delete

> models::HealthRecordsDisconnectResponse disconnect_health_records_api_evidence_health_records_delete()
Disconnect Health Records

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HealthRecordsDisconnectResponse**](HealthRecordsDisconnectResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disconnect_oracle_api_connectors_oracle_organization_id_delete

> models::EpicDisconnectResponse disconnect_oracle_api_connectors_oracle_organization_id_delete(organization_id)
Disconnect Oracle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

[**models::EpicDisconnectResponse**](EpicDisconnectResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evidence_lab_api_evidence_labs_post

> models::EvidenceLabPage evidence_lab_api_evidence_labs_post(evidence_lab_query)
Evidence Lab

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evidence_lab_query** | [**EvidenceLabQuery**](EvidenceLabQuery.md) |  | [required] |

### Return type

[**models::EvidenceLabPage**](EvidenceLabPage.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evidence_library_api_evidence_library_post

> models::EvidenceLibraryPage evidence_library_api_evidence_library_post(evidence_library_query)
Evidence Library

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evidence_library_query** | [**EvidenceLibraryQuery**](EvidenceLibraryQuery.md) |  | [required] |

### Return type

[**models::EvidenceLibraryPage**](EvidenceLibraryPage.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evidence_records_api_evidence_records_get

> models::EvidenceReadPage evidence_records_api_evidence_records_get(entity, record_ids, concept_id, source_kind, clinical_kind, start, end, cursor)
Evidence Records

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity** | **String** |  | [required] |
**record_ids** | Option<**Vec<uuid::Uuid>**> |  |  |
**concept_id** | Option<**String**> |  |  |
**source_kind** | Option<**String**> |  |  |
**clinical_kind** | Option<**String**> |  |  |
**start** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  |  |
**end** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  |  |
**cursor** | Option<**String**> |  |  |

### Return type

[**models::EvidenceReadPage**](EvidenceReadPage.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evidence_series_api_evidence_series_post

> models::EvidenceSeriesPage evidence_series_api_evidence_series_post(evidence_series_query, x_duet_app_build)
Evidence Series

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evidence_series_query** | [**EvidenceSeriesQuery**](EvidenceSeriesQuery.md) |  | [required] |
**x_duet_app_build** | Option<**i32**> |  |  |

### Return type

[**models::EvidenceSeriesPage**](EvidenceSeriesPage.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evidence_state_api_evidence_state_get

> models::EvidenceStateResponse evidence_state_api_evidence_state_get(cursor, since_revision, catalog_revision, source_generation)
Evidence State

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> |  |  |
**since_revision** | Option<**i32**> |  |  |
**catalog_revision** | Option<**String**> |  |  |
**source_generation** | Option<**i32**> |  |  |

### Return type

[**models::EvidenceStateResponse**](EvidenceStateResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evidence_timeline_api_evidence_timeline_post

> models::EvidenceTimelinePage evidence_timeline_api_evidence_timeline_post(evidence_timeline_query)
Evidence Timeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evidence_timeline_query** | [**EvidenceTimelineQuery**](EvidenceTimelineQuery.md) |  | [required] |

### Return type

[**models::EvidenceTimelinePage**](EvidenceTimelinePage.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_approved_doctor_page

> std::path::PathBuf export_approved_doctor_page()


### Parameters

This endpoint does not need any parameter.

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_health_get

> std::collections::HashMap<String, String> health_health_get()
Health

### Parameters

This endpoint does not need any parameter.

### Return type

**std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_records_source_api_evidence_health_records_get

> models::PhoneHealthSourceResponse health_records_source_api_evidence_health_records_get(x_duet_source_key)
Health Records Source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_duet_source_key** | Option<**String**> |  |  |

### Return type

[**models::PhoneHealthSourceResponse**](PhoneHealthSourceResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingest_canonical_evidence

> models::CanonicalSourceAddress ingest_canonical_evidence(canonical_evidence_mutation)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**canonical_evidence_mutation** | [**CanonicalEvidenceMutation**](CanonicalEvidenceMutation.md) |  | [required] |

### Return type

[**models::CanonicalSourceAddress**](CanonicalSourceAddress.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingest_evidence_api_evidence_post

> models::EvidenceBatchResponse ingest_evidence_api_evidence_post(evidence_batch_request)
Ingest Evidence

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evidence_batch_request** | [**EvidenceBatchRequest**](EvidenceBatchRequest.md) |  | [required] |

### Return type

[**models::EvidenceBatchResponse**](EvidenceBatchResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portal_organizations_api_connectors_portals_organizations_get

> models::PortalOrganizationsResponse portal_organizations_api_connectors_portals_organizations_get(query)
Portal Organizations

One search across every available patient-portal vendor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |[default to ]

### Return type

[**models::PortalOrganizationsResponse**](PortalOrganizationsResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portal_status_api_connectors_portals_get

> models::PortalsResponse portal_status_api_connectors_portals_get()
Portal Status

The vendors offered for new connections, and every patient-portal connection the person has, whether or not its vendor is still offered.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PortalsResponse**](PortalsResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## preview_approved_doctor_page

> String preview_approved_doctor_page()


### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/svg+xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_approved_doctor_page

> models::DurablePublicationView publish_approved_doctor_page(durable_publish_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**durable_publish_request** | [**DurablePublishRequest**](DurablePublishRequest.md) |  | [required] |

### Return type

[**models::DurablePublicationView**](DurablePublicationView.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_canonical_evidence_exact

> models::CanonicalExactEvidence read_canonical_evidence_exact(read_canonical_evidence_exact_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_canonical_evidence_exact_request** | [**ReadCanonicalEvidenceExactRequest**](ReadCanonicalEvidenceExactRequest.md) |  | [required] |

### Return type

[**models::CanonicalExactEvidence**](CanonicalExactEvidence.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_canonical_evidence_snapshot

> models::CanonicalEvidenceSnapshot read_canonical_evidence_snapshot(after)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**i64**> |  |  |[default to 0]

### Return type

[**models::CanonicalEvidenceSnapshot**](CanonicalEvidenceSnapshot.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_canonical_health_context

> read_canonical_health_context()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_doctor_publication

> models::DurablePublicationView read_doctor_publication()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DurablePublicationView**](DurablePublicationView.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_doctor_save_receipt

> models::DoctorCollection read_doctor_save_receipt(generation)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**generation** | **i64** |  | [required] |

### Return type

[**models::DoctorCollection**](DoctorCollection.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_durable_account_context

> models::ReadDurableAccountContext200Response read_durable_account_context()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ReadDurableAccountContext200Response**](ReadDurableAccountContext200Response.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_durable_conversation

> std::collections::HashMap<String, serde_json::Value> read_durable_conversation(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

**std::collections::HashMap<String, serde_json::Value>**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_durable_conversation_events

> String read_durable_conversation_events(id, last_event_id, after)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**last_event_id** | Option<**i64**> |  |  |
**after** | Option<**i64**> |  |  |

### Return type

**String**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_durable_conversation_usage

> std::collections::HashMap<String, serde_json::Value> read_durable_conversation_usage(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**uuid::Uuid**> |  | [required] |

### Return type

**std::collections::HashMap<String, serde_json::Value>**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_today_assessment_status

> read_today_assessment_status(durable_today_slot)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**durable_today_slot** | [**DurableTodaySlot**](DurableTodaySlot.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reject_durable_doctor_proposal

> models::DoctorCollection reject_durable_doctor_proposal(id, reject_durable_doctor_proposal_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**reject_durable_doctor_proposal_request** | [**RejectDurableDoctorProposalRequest**](RejectDurableDoctorProposalRequest.md) |  | [required] |

### Return type

[**models::DoctorCollection**](DoctorCollection.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_durable_conversation_command

> resume_durable_conversation_command(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_published_doctor_pdf

> std::path::PathBuf retrieve_published_doctor_pdf(token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_smart_health_link_file

> String retrieve_smart_health_link_file(location)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/jose

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_smart_health_link_manifest

> std::collections::HashMap<String, serde_json::Value> retrieve_smart_health_link_manifest(token, request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |
**request_body** | **std::collections::HashMap<String, serde_json::Value>** |  | [required] |

### Return type

**std::collections::HashMap<String, serde_json::Value>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_doctor_publication

> revoke_doctor_publication(idempotency_key, if_match)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**uuid::Uuid**> |  | [required] |
**if_match** | Option<**uuid::Uuid**> | Bare UUID of the reviewed publication, not a quoted ETag or wildcard. Retry with the same operation and target after uncertain acknowledgement. | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_epic_api_connectors_epic_start_post

> models::EpicAuthorizationResponse start_epic_api_connectors_epic_start_post(epic_authorization_request)
Start Epic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_authorization_request** | [**EpicAuthorizationRequest**](EpicAuthorizationRequest.md) |  | [required] |

### Return type

[**models::EpicAuthorizationResponse**](EpicAuthorizationResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_healow_api_connectors_healow_start_post

> models::EpicAuthorizationResponse start_healow_api_connectors_healow_start_post(epic_authorization_request)
Start Healow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_authorization_request** | [**EpicAuthorizationRequest**](EpicAuthorizationRequest.md) |  | [required] |

### Return type

[**models::EpicAuthorizationResponse**](EpicAuthorizationResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_oracle_api_connectors_oracle_start_post

> models::EpicAuthorizationResponse start_oracle_api_connectors_oracle_start_post(epic_authorization_request)
Start Oracle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_authorization_request** | [**EpicAuthorizationRequest**](EpicAuthorizationRequest.md) |  | [required] |

### Return type

[**models::EpicAuthorizationResponse**](EpicAuthorizationResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_epic_api_connectors_epic_organization_id_sync_post

> models::PortalConnectionResponse sync_epic_api_connectors_epic_organization_id_sync_post(organization_id)
Sync Epic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

[**models::PortalConnectionResponse**](PortalConnectionResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_healow_api_connectors_healow_organization_id_sync_post

> models::PortalConnectionResponse sync_healow_api_connectors_healow_organization_id_sync_post(organization_id)
Sync Healow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

[**models::PortalConnectionResponse**](PortalConnectionResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_oracle_api_connectors_oracle_organization_id_sync_post

> models::PortalConnectionResponse sync_oracle_api_connectors_oracle_organization_id_sync_post(organization_id)
Sync Oracle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

[**models::PortalConnectionResponse**](PortalConnectionResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

