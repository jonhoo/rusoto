// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>A container for account-level settings within AWS Device Farm.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AccountSettings {
    /// <p>The AWS account number specified in the <code>AccountSettings</code> container.</p>
    #[serde(rename = "awsAccountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_number: Option<String>,
    /// <p>The default number of minutes (at the account level) a test run will execute before it times out. Default value is 60 minutes.</p>
    #[serde(rename = "defaultJobTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_job_timeout_minutes: Option<i64>,
    /// <p>The maximum number of minutes a test run will execute before it times out.</p>
    #[serde(rename = "maxJobTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_job_timeout_minutes: Option<i64>,
    /// <p>The maximum number of device slots that the AWS account can purchase. Each maximum is expressed as an <code>offering-id:number</code> pair, where the <code>offering-id</code> represents one of the IDs returned by the <code>ListOfferings</code> command.</p>
    #[serde(rename = "maxSlots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_slots: Option<::std::collections::HashMap<String, i64>>,
    /// <p>When set to <code>true</code>, for private devices, Device Farm will not sign your app again. For public devices, Device Farm always signs your apps again and this parameter has no effect.</p> <p>For more information about how Device Farm re-signs your app(s), see <a href="https://aws.amazon.com/device-farm/faq/">Do you modify my app?</a> in the <i>AWS Device Farm FAQs</i>.</p>
    #[serde(rename = "skipAppResign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_app_resign: Option<bool>,
    /// <p>Information about an AWS account's usage of free trial device minutes.</p>
    #[serde(rename = "trialMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_minutes: Option<TrialMinutes>,
    /// <p>Returns the unmetered devices you have purchased or want to purchase.</p>
    #[serde(rename = "unmeteredDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmetered_devices: Option<::std::collections::HashMap<String, i64>>,
    /// <p>Returns the unmetered remote access devices you have purchased or want to purchase.</p>
    #[serde(rename = "unmeteredRemoteAccessDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmetered_remote_access_devices: Option<::std::collections::HashMap<String, i64>>,
}

/// <p>An invalid argument was specified.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ArgumentException {
    /// <p>Any additional information about the exception.</p>
    pub message: Option<String>,
}

/// <p>Represents the output of a test. Examples of artifacts include logs and screenshots.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Artifact {
    /// <p>The artifact's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The artifact's file extension.</p>
    #[serde(rename = "extension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    /// <p>The artifact's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The artifact&#39;s type.</p> <p>Allowed values include the following:</p> <ul> <li> <p>UNKNOWN: An unknown type.</p> </li> <li> <p>SCREENSHOT: The screenshot type.</p> </li> <li> <p>DEVICE<em>LOG: The device log type.</p> </li> <li> <p>MESSAGE</em>LOG: The message log type.</p> </li> <li> <p>RESULT<em>LOG: The result log type.</p> </li> <li> <p>SERVICE</em>LOG: The service log type.</p> </li> <li> <p>WEBKIT<em>LOG: The web kit log type.</p> </li> <li> <p>INSTRUMENTATION</em>OUTPUT: The instrumentation type.</p> </li> <li> <p>EXERCISER<em>MONKEY</em>OUTPUT: For Android, the artifact (log) generated by an Android fuzz test.</p> </li> <li> <p>CALABASH<em>JSON</em>OUTPUT: The Calabash JSON output type.</p> </li> <li> <p>CALABASH<em>PRETTY</em>OUTPUT: The Calabash pretty output type.</p> </li> <li> <p>CALABASH<em>STANDARD</em>OUTPUT: The Calabash standard output type.</p> </li> <li> <p>CALABASH<em>JAVA</em>XML<em>OUTPUT: The Calabash Java XML output type.</p> </li> <li> <p>AUTOMATION</em>OUTPUT: The automation output type.</p> </li> <li> <p>APPIUM<em>SERVER</em>OUTPUT: The Appium server output type.</p> </li> <li> <p>APPIUM<em>JAVA</em>OUTPUT: The Appium Java output type.</p> </li> <li> <p>APPIUM<em>JAVA</em>XML<em>OUTPUT: The Appium Java XML output type.</p> </li> <li> <p>APPIUM</em>PYTHON<em>OUTPUT: The Appium Python output type.</p> </li> <li> <p>APPIUM</em>PYTHON<em>XML</em>OUTPUT: The Appium Python XML output type.</p> </li> <li> <p>EXPLORER<em>EVENT</em>LOG: The Explorer event log output type.</p> </li> <li> <p>EXPLORER<em>SUMMARY</em>LOG: The Explorer summary log output type.</p> </li> <li> <p>APPLICATION<em>CRASH</em>REPORT: The application crash report output type.</p> </li> <li> <p>XCTEST_LOG: The XCode test output type.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The pre-signed Amazon S3 URL that can be used with a corresponding GET request to download the artifact's file.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Represents the amount of CPU that an app is using on a physical device.</p> <p>Note that this does not represent system-wide CPU usage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CPU {
    /// <p>The CPU's architecture, for example x86 or ARM.</p>
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// <p>The clock speed of the device's CPU, expressed in hertz (Hz). For example, a 1.2 GHz CPU is expressed as 1200000000.</p>
    #[serde(rename = "clock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock: Option<f64>,
    /// <p>The CPU's frequency.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
}

/// <p>Represents entity counters.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Counters {
    /// <p>The number of errored entities.</p>
    #[serde(rename = "errored")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errored: Option<i64>,
    /// <p>The number of failed entities.</p>
    #[serde(rename = "failed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    /// <p>The number of passed entities.</p>
    #[serde(rename = "passed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passed: Option<i64>,
    /// <p>The number of skipped entities.</p>
    #[serde(rename = "skipped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<i64>,
    /// <p>The number of stopped entities.</p>
    #[serde(rename = "stopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<i64>,
    /// <p>The total number of entities.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// <p>The number of warned entities.</p>
    #[serde(rename = "warned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warned: Option<i64>,
}

/// <p>Represents a request to the create device pool operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDevicePoolRequest {
    /// <p>The device pool's description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The device pool's name.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ARN of the project for the device pool.</p>
    #[serde(rename = "projectArn")]
    pub project_arn: String,
    /// <p>The device pool's rules.</p>
    #[serde(rename = "rules")]
    pub rules: Vec<Rule>,
}

/// <p>Represents the result of a create device pool request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDevicePoolResult {
    /// <p>The newly created device pool.</p>
    #[serde(rename = "devicePool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_pool: Option<DevicePool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateInstanceProfileRequest {
    /// <p>The description of your instance profile.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An array of strings specifying the list of app packages that should not be cleaned up from the device after a test run is over.</p> <p>The list of packages is only considered if you set <code>packageCleanup</code> to <code>true</code>.</p>
    #[serde(rename = "excludeAppPackagesFromCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_app_packages_from_cleanup: Option<Vec<String>>,
    /// <p>The name of your instance profile.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>When set to <code>true</code>, Device Farm will remove app packages after a test run. The default value is <code>false</code> for private devices.</p>
    #[serde(rename = "packageCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_cleanup: Option<bool>,
    /// <p>When set to <code>true</code>, Device Farm will reboot the instance after a test run. The default value is <code>true</code>.</p>
    #[serde(rename = "rebootAfterUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reboot_after_use: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateInstanceProfileResult {
    /// <p>An object containing information about your instance profile.</p>
    #[serde(rename = "instanceProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile: Option<InstanceProfile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateNetworkProfileRequest {
    /// <p>The description of the network profile.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    #[serde(rename = "downlinkBandwidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_bandwidth_bits: Option<i64>,
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "downlinkDelayMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_delay_ms: Option<i64>,
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "downlinkJitterMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_jitter_ms: Option<i64>,
    /// <p>Proportion of received packets that fail to arrive from 0 to 100 percent.</p>
    #[serde(rename = "downlinkLossPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_loss_percent: Option<i64>,
    /// <p>The name you wish to specify for the new network profile.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to create a network profile.</p>
    #[serde(rename = "projectArn")]
    pub project_arn: String,
    /// <p>The type of network profile you wish to create. Valid values are listed below.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    #[serde(rename = "uplinkBandwidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_bandwidth_bits: Option<i64>,
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "uplinkDelayMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_delay_ms: Option<i64>,
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "uplinkJitterMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_jitter_ms: Option<i64>,
    /// <p>Proportion of transmitted packets that fail to arrive from 0 to 100 percent.</p>
    #[serde(rename = "uplinkLossPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_loss_percent: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateNetworkProfileResult {
    /// <p>The network profile that is returned by the create network profile request.</p>
    #[serde(rename = "networkProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
}

/// <p>Represents a request to the create project operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateProjectRequest {
    /// <p>Sets the execution timeout value (in minutes) for a project. All test runs in this project will use the specified execution timeout value unless overridden when scheduling a run.</p>
    #[serde(rename = "defaultJobTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_job_timeout_minutes: Option<i64>,
    /// <p>The project's name.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Represents the result of a create project request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateProjectResult {
    /// <p>The newly created project.</p>
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
}

/// <p>Configuration settings for a remote access session, including billing method.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRemoteAccessSessionConfiguration {
    /// <p>The billing method for the remote access session.</p>
    #[serde(rename = "billingMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_method: Option<String>,
    /// <p>An array of Amazon Resource Names (ARNs) included in the VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfigurationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration_arns: Option<Vec<String>>,
}

/// <p>Creates and submits a request to start a remote access session.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRemoteAccessSessionRequest {
    /// <p>Unique identifier for the client. If you want access to multiple devices on the same client, you should pass the same <code>clientId</code> value in each call to <code>CreateRemoteAccessSession</code>. This is required only if <code>remoteDebugEnabled</code> is set to <code>true</code>.</p>
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The configuration information for the remote access session request.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<CreateRemoteAccessSessionConfiguration>,
    /// <p>The Amazon Resource Name (ARN) of the device for which you want to create a remote access session.</p>
    #[serde(rename = "deviceArn")]
    pub device_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the device instance for which you want to create a remote access session.</p>
    #[serde(rename = "instanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    /// <p><p>The interaction mode of the remote access session. Valid values are:</p> <ul> <li> <p>INTERACTIVE: You can interact with the iOS device by viewing, touching, and rotating the screen. You <b>cannot</b> run XCUITest framework-based tests in this mode.</p> </li> <li> <p>NO<em>VIDEO: You are connected to the device but cannot interact with it or view the screen. This mode has the fastest test execution speed. You <b>can</b> run XCUITest framework-based tests in this mode.</p> </li> <li> <p>VIDEO</em>ONLY: You can view the screen but cannot touch or rotate it. You <b>can</b> run XCUITest framework-based tests and watch the screen in this mode.</p> </li> </ul></p>
    #[serde(rename = "interactionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_mode: Option<String>,
    /// <p>The name of the remote access session that you wish to create.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to create a remote access session.</p>
    #[serde(rename = "projectArn")]
    pub project_arn: String,
    /// <p>Set to <code>true</code> if you want to access devices remotely for debugging in your remote access session.</p>
    #[serde(rename = "remoteDebugEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_debug_enabled: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) for the app to be recorded in the remote access session.</p>
    #[serde(rename = "remoteRecordAppArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_record_app_arn: Option<String>,
    /// <p>Set to <code>true</code> to enable remote recording for the remote access session.</p>
    #[serde(rename = "remoteRecordEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_record_enabled: Option<bool>,
    /// <p>When set to <code>true</code>, for private devices, Device Farm will not sign your app again. For public devices, Device Farm always signs your apps again and this parameter has no effect.</p> <p>For more information about how Device Farm re-signs your app(s), see <a href="https://aws.amazon.com/device-farm/faq/">Do you modify my app?</a> in the <i>AWS Device Farm FAQs</i>.</p>
    #[serde(rename = "skipAppResign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_app_resign: Option<bool>,
    /// <p>The public key of the <code>ssh</code> key pair you want to use for connecting to remote devices in your remote debugging session. This is only required if <code>remoteDebugEnabled</code> is set to <code>true</code>.</p>
    #[serde(rename = "sshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
}

/// <p>Represents the server response from a request to create a remote access session.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateRemoteAccessSessionResult {
    /// <p>A container that describes the remote access session when the request to create a remote access session is sent.</p>
    #[serde(rename = "remoteAccessSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_session: Option<RemoteAccessSession>,
}

/// <p>Represents a request to the create upload operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUploadRequest {
    /// <p>The upload's content type (for example, "application/octet-stream").</p>
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The upload's file name. The name should not contain the '/' character. If uploading an iOS app, the file name needs to end with the <code>.ipa</code> extension. If uploading an Android app, the file name needs to end with the <code>.apk</code> extension. For all others, the file name must end with the <code>.zip</code> file extension.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ARN of the project for the upload.</p>
    #[serde(rename = "projectArn")]
    pub project_arn: String,
    /// <p>The upload's upload type.</p> <p>Must be one of the following values:</p> <ul> <li> <p>ANDROID_APP: An Android upload.</p> </li> <li> <p>IOS_APP: An iOS upload.</p> </li> <li> <p>WEB_APP: A web application upload.</p> </li> <li> <p>EXTERNAL_DATA: An external data upload.</p> </li> <li> <p>APPIUM_JAVA_JUNIT_TEST_PACKAGE: An Appium Java JUnit test package upload.</p> </li> <li> <p>APPIUM_JAVA_TESTNG_TEST_PACKAGE: An Appium Java TestNG test package upload.</p> </li> <li> <p>APPIUM_PYTHON_TEST_PACKAGE: An Appium Python test package upload.</p> </li> <li> <p>APPIUM_WEB_JAVA_JUNIT_TEST_PACKAGE: An Appium Java JUnit test package upload.</p> </li> <li> <p>APPIUM_WEB_JAVA_TESTNG_TEST_PACKAGE: An Appium Java TestNG test package upload.</p> </li> <li> <p>APPIUM_WEB_PYTHON_TEST_PACKAGE: An Appium Python test package upload.</p> </li> <li> <p>CALABASH_TEST_PACKAGE: A Calabash test package upload.</p> </li> <li> <p>INSTRUMENTATION_TEST_PACKAGE: An instrumentation upload.</p> </li> <li> <p>UIAUTOMATION_TEST_PACKAGE: A uiautomation test package upload.</p> </li> <li> <p>UIAUTOMATOR_TEST_PACKAGE: A uiautomator test package upload.</p> </li> <li> <p>XCTEST_TEST_PACKAGE: An XCode test package upload.</p> </li> <li> <p>XCTEST_UI_TEST_PACKAGE: An XCode UI test package upload.</p> </li> </ul> <p> <b>Note</b> If you call <code>CreateUpload</code> with <code>WEB_APP</code> specified, AWS Device Farm throws an <code>ArgumentException</code> error.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Represents the result of a create upload request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateUploadResult {
    /// <p>The newly created upload.</p>
    #[serde(rename = "upload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<Upload>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateVPCEConfigurationRequest {
    /// <p>The DNS name of the service running in your VPC that you want Device Farm to test.</p>
    #[serde(rename = "serviceDnsName")]
    pub service_dns_name: String,
    /// <p>An optional description, providing more details about your VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration_description: Option<String>,
    /// <p>The friendly name you give to your VPC endpoint configuration, to manage your configurations more easily.</p>
    #[serde(rename = "vpceConfigurationName")]
    pub vpce_configuration_name: String,
    /// <p>The name of the VPC endpoint service running inside your AWS account that you want Device Farm to test.</p>
    #[serde(rename = "vpceServiceName")]
    pub vpce_service_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateVPCEConfigurationResult {
    /// <p>An object containing information about your VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration: Option<VPCEConfiguration>,
}

/// <p>A JSON object specifying the paths where the artifacts generated by the customer's tests, on the device or in the test environment, will be pulled from.</p> <p>Specify <code>deviceHostPaths</code> and optionally specify either <code>iosPaths</code> or <code>androidPaths</code>.</p> <p>For web app tests, you can specify both <code>iosPaths</code> and <code>androidPaths</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerArtifactPaths {
    /// <p>Comma-separated list of paths on the Android device where the artifacts generated by the customer's tests will be pulled from.</p>
    #[serde(rename = "androidPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_paths: Option<Vec<String>>,
    /// <p>Comma-separated list of paths in the test execution environment where the artifacts generated by the customer's tests will be pulled from.</p>
    #[serde(rename = "deviceHostPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_host_paths: Option<Vec<String>>,
    /// <p>Comma-separated list of paths on the iOS device where the artifacts generated by the customer's tests will be pulled from.</p>
    #[serde(rename = "iosPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_paths: Option<Vec<String>>,
}

/// <p>Represents a request to the delete device pool operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDevicePoolRequest {
    /// <p>Represents the Amazon Resource Name (ARN) of the Device Farm device pool you wish to delete.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a delete device pool request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteDevicePoolResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteInstanceProfileRequest {
    /// <p>The Amazon Resource Name (ARN) of the instance profile you are requesting to delete.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteInstanceProfileResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteNetworkProfileRequest {
    /// <p>The Amazon Resource Name (ARN) of the network profile you want to delete.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteNetworkProfileResult {}

/// <p>Represents a request to the delete project operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteProjectRequest {
    /// <p>Represents the Amazon Resource Name (ARN) of the Device Farm project you wish to delete.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a delete project request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteProjectResult {}

/// <p>Represents the request to delete the specified remote access session.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRemoteAccessSessionRequest {
    /// <p>The Amazon Resource Name (ARN) of the sesssion for which you want to delete remote access.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>The response from the server when a request is made to delete the remote access session.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteRemoteAccessSessionResult {}

/// <p>Represents a request to the delete run operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRunRequest {
    /// <p>The Amazon Resource Name (ARN) for the run you wish to delete.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a delete run request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteRunResult {}

/// <p>Represents a request to the delete upload operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUploadRequest {
    /// <p>Represents the Amazon Resource Name (ARN) of the Device Farm upload you wish to delete.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a delete upload request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteUploadResult {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVPCEConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the VPC endpoint configuration you want to delete.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteVPCEConfigurationResult {}

/// <p>Represents a device type that an app is tested against.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Device {
    /// <p>The device's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The device's carrier.</p>
    #[serde(rename = "carrier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// <p>Information about the device's CPU.</p>
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<CPU>,
    /// <p>The name of the fleet to which this device belongs.</p>
    #[serde(rename = "fleetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_name: Option<String>,
    /// <p>The type of fleet to which this device belongs. Possible values for fleet type are PRIVATE and PUBLIC.</p>
    #[serde(rename = "fleetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_type: Option<String>,
    /// <p><p>The device&#39;s form factor.</p> <p>Allowed values include:</p> <ul> <li> <p>PHONE: The phone form factor.</p> </li> <li> <p>TABLET: The tablet form factor.</p> </li> </ul></p>
    #[serde(rename = "formFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_factor: Option<String>,
    /// <p>The device's heap size, expressed in bytes.</p>
    #[serde(rename = "heapSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heap_size: Option<i64>,
    /// <p>The device's image name.</p>
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// <p>The instances belonging to this device.</p>
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<DeviceInstance>>,
    /// <p>The device's manufacturer name.</p>
    #[serde(rename = "manufacturer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// <p>The device's total memory size, expressed in bytes.</p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// <p>The device's model name.</p>
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// <p>The device's model ID.</p>
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// <p>The device's display name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The device's operating system type.</p>
    #[serde(rename = "os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// <p><p>The device&#39;s platform.</p> <p>Allowed values include:</p> <ul> <li> <p>ANDROID: The Android platform.</p> </li> <li> <p>IOS: The iOS platform.</p> </li> </ul></p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The device's radio.</p>
    #[serde(rename = "radio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radio: Option<String>,
    /// <p>Specifies whether remote access has been enabled for the specified device.</p>
    #[serde(rename = "remoteAccessEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_enabled: Option<bool>,
    /// <p>This flag is set to <code>true</code> if remote debugging is enabled for the device.</p>
    #[serde(rename = "remoteDebugEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_debug_enabled: Option<bool>,
    /// <p>The resolution of the device.</p>
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
}

/// <p>Represents the device instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeviceInstance {
    /// <p>The Amazon Resource Name (ARN) of the device instance.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the device.</p>
    #[serde(rename = "deviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
    /// <p>A object containing information about the instance profile.</p>
    #[serde(rename = "instanceProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile: Option<InstanceProfile>,
    /// <p>An array of strings describing the device instance.</p>
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>The status of the device instance. Valid values are listed below.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Unique device identifier for the device instance.</p>
    #[serde(rename = "udid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udid: Option<String>,
}

/// <p>Represents the total (metered or unmetered) minutes used by the resource to run tests. Contains the sum of minutes consumed by all children.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeviceMinutes {
    /// <p>When specified, represents only the sum of metered minutes used by the resource to run tests.</p>
    #[serde(rename = "metered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered: Option<f64>,
    /// <p>When specified, represents the total minutes used by the resource to run tests.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    /// <p>When specified, represents only the sum of unmetered minutes used by the resource to run tests.</p>
    #[serde(rename = "unmetered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmetered: Option<f64>,
}

/// <p>Represents a collection of device types.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DevicePool {
    /// <p>The device pool's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The device pool's description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The device pool's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Information about the device pool's rules.</p>
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    /// <p><p>The device pool&#39;s type.</p> <p>Allowed values include:</p> <ul> <li> <p>CURATED: A device pool that is created and managed by AWS Device Farm.</p> </li> <li> <p>PRIVATE: A device pool that is created and managed by the device pool developer.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents a device pool compatibility result.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DevicePoolCompatibilityResult {
    /// <p>Whether the result was compatible with the device pool.</p>
    #[serde(rename = "compatible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible: Option<bool>,
    /// <p>The device (phone or tablet) that you wish to return information about.</p>
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    /// <p>Information about the compatibility.</p>
    #[serde(rename = "incompatibilityMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incompatibility_messages: Option<Vec<IncompatibilityMessage>>,
}

/// <p>Represents configuration information about a test run, such as the execution timeout (in minutes).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExecutionConfiguration {
    /// <p>True if account cleanup is enabled at the beginning of the test; otherwise, false.</p>
    #[serde(rename = "accountsCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts_cleanup: Option<bool>,
    /// <p>True if app package cleanup is enabled at the beginning of the test; otherwise, false.</p>
    #[serde(rename = "appPackagesCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_packages_cleanup: Option<bool>,
    /// <p>The number of minutes a test run will execute before it times out.</p>
    #[serde(rename = "jobTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_timeout_minutes: Option<i64>,
    /// <p>When set to <code>true</code>, for private devices, Device Farm will not sign your app again. For public devices, Device Farm always signs your apps again and this parameter has no effect.</p> <p>For more information about how Device Farm re-signs your app(s), see <a href="https://aws.amazon.com/device-farm/faq/">Do you modify my app?</a> in the <i>AWS Device Farm FAQs</i>.</p>
    #[serde(rename = "skipAppResign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_app_resign: Option<bool>,
    /// <p>Set to true to enable video capture; otherwise, set to false. The default is true.</p>
    #[serde(rename = "videoCapture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_capture: Option<bool>,
}

/// <p>Represents the request sent to retrieve the account settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAccountSettingsRequest {}

/// <p>Represents the account settings return values from the <code>GetAccountSettings</code> request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetAccountSettingsResult {
    /// <p>The account settings.</p>
    #[serde(rename = "accountSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_settings: Option<AccountSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeviceInstanceRequest {
    /// <p>The Amazon Resource Name (ARN) of the instance you're requesting information about.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDeviceInstanceResult {
    /// <p>An object containing information about your device instance.</p>
    #[serde(rename = "deviceInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_instance: Option<DeviceInstance>,
}

/// <p>Represents a request to the get device pool compatibility operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDevicePoolCompatibilityRequest {
    /// <p>The ARN of the app that is associated with the specified device pool.</p>
    #[serde(rename = "appArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    /// <p>An object containing information about the settings for a run.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ScheduleRunConfiguration>,
    /// <p>The device pool's ARN.</p>
    #[serde(rename = "devicePoolArn")]
    pub device_pool_arn: String,
    /// <p>Information about the uploaded test to be run against the device pool.</p>
    #[serde(rename = "test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<ScheduleRunTest>,
    /// <p><p>The test type for the specified device pool.</p> <p>Allowed values include the following:</p> <ul> <li> <p>BUILTIN<em>FUZZ: The built-in fuzz type.</p> </li> <li> <p>BUILTIN</em>EXPLORER: For Android, an app explorer that will traverse an Android app, interacting with it and capturing screenshots at the same time.</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT: The Appium Java JUnit type.</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG: The Appium Java TestNG type.</p> </li> <li> <p>APPIUM<em>PYTHON: The Appium Python type.</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT: The Appium Java JUnit type for Web apps.</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG: The Appium Java TestNG type for Web apps.</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON: The Appium Python type for Web apps.</p> </li> <li> <p>CALABASH: The Calabash type.</p> </li> <li> <p>INSTRUMENTATION: The Instrumentation type.</p> </li> <li> <p>UIAUTOMATION: The uiautomation type.</p> </li> <li> <p>UIAUTOMATOR: The uiautomator type.</p> </li> <li> <p>XCTEST: The XCode test type.</p> </li> <li> <p>XCTEST</em>UI: The XCode UI test type.</p> </li> </ul></p>
    #[serde(rename = "testType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_type: Option<String>,
}

/// <p>Represents the result of describe device pool compatibility request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDevicePoolCompatibilityResult {
    /// <p>Information about compatible devices.</p>
    #[serde(rename = "compatibleDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_devices: Option<Vec<DevicePoolCompatibilityResult>>,
    /// <p>Information about incompatible devices.</p>
    #[serde(rename = "incompatibleDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incompatible_devices: Option<Vec<DevicePoolCompatibilityResult>>,
}

/// <p>Represents a request to the get device pool operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDevicePoolRequest {
    /// <p>The device pool's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get device pool request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDevicePoolResult {
    /// <p>An object containing information about the requested device pool.</p>
    #[serde(rename = "devicePool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_pool: Option<DevicePool>,
}

/// <p>Represents a request to the get device request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeviceRequest {
    /// <p>The device type's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get device request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetDeviceResult {
    /// <p>An object containing information about the requested device.</p>
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInstanceProfileRequest {
    /// <p>The Amazon Resource Name (ARN) of your instance profile.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetInstanceProfileResult {
    /// <p>An object containing information about your instance profile.</p>
    #[serde(rename = "instanceProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile: Option<InstanceProfile>,
}

/// <p>Represents a request to the get job operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetJobRequest {
    /// <p>The job's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get job request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetJobResult {
    /// <p>An object containing information about the requested job.</p>
    #[serde(rename = "job")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetNetworkProfileRequest {
    /// <p>The Amazon Resource Name (ARN) of the network profile you want to return information about.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetNetworkProfileResult {
    /// <p>The network profile.</p>
    #[serde(rename = "networkProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
}

/// <p>Represents the request to retrieve the offering status for the specified customer or account.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetOfferingStatusRequest {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Returns the status result for a device offering.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetOfferingStatusResult {
    /// <p>When specified, gets the offering status for the current period.</p>
    #[serde(rename = "current")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<::std::collections::HashMap<String, OfferingStatus>>,
    /// <p>When specified, gets the offering status for the next period.</p>
    #[serde(rename = "nextPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_period: Option<::std::collections::HashMap<String, OfferingStatus>>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents a request to the get project operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetProjectRequest {
    /// <p>The project's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get project request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetProjectResult {
    /// <p>The project you wish to get information about.</p>
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
}

/// <p>Represents the request to get information about the specified remote access session.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRemoteAccessSessionRequest {
    /// <p>The Amazon Resource Name (ARN) of the remote access session about which you want to get session information.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the response from the server that lists detailed information about the remote access session.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetRemoteAccessSessionResult {
    /// <p>A container that lists detailed information about the remote access session.</p>
    #[serde(rename = "remoteAccessSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_session: Option<RemoteAccessSession>,
}

/// <p>Represents a request to the get run operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRunRequest {
    /// <p>The run's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get run request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetRunResult {
    /// <p>The run you wish to get results from.</p>
    #[serde(rename = "run")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<Run>,
}

/// <p>Represents a request to the get suite operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSuiteRequest {
    /// <p>The suite's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get suite request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetSuiteResult {
    /// <p>A collection of one or more tests.</p>
    #[serde(rename = "suite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suite: Option<Suite>,
}

/// <p>Represents a request to the get test operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTestRequest {
    /// <p>The test's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get test request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetTestResult {
    /// <p>A test condition that is evaluated.</p>
    #[serde(rename = "test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<Test>,
}

/// <p>Represents a request to the get upload operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUploadRequest {
    /// <p>The upload's ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the result of a get upload request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetUploadResult {
    /// <p>An app or a set of one or more tests to upload or that have been uploaded.</p>
    #[serde(rename = "upload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<Upload>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetVPCEConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the VPC endpoint configuration you want to describe.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetVPCEConfigurationResult {
    /// <p>An object containing information about your VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration: Option<VPCEConfiguration>,
}

/// <p>An entity with the same name already exists.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IdempotencyException {
    /// <p>Any additional information about the exception.</p>
    pub message: Option<String>,
}

/// <p>Represents information about incompatibility.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct IncompatibilityMessage {
    /// <p>A message about the incompatibility.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p><p>The type of incompatibility.</p> <p>Allowed values include:</p> <ul> <li> <p>ARN: The ARN.</p> </li> <li> <p>FORM<em>FACTOR: The form factor (for example, phone or tablet).</p> </li> <li> <p>MANUFACTURER: The manufacturer.</p> </li> <li> <p>PLATFORM: The platform (for example, Android or iOS).</p> </li> <li> <p>REMOTE</em>ACCESS<em>ENABLED: Whether the device is enabled for remote access.</p> </li> <li> <p>APPIUM</em>VERSION: The Appium version for the test.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents the request to install an Android application (in .apk format) or an iOS application (in .ipa format) as part of a remote access session.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InstallToRemoteAccessSessionRequest {
    /// <p>The Amazon Resource Name (ARN) of the app about which you are requesting information.</p>
    #[serde(rename = "appArn")]
    pub app_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the remote access session about which you are requesting information.</p>
    #[serde(rename = "remoteAccessSessionArn")]
    pub remote_access_session_arn: String,
}

/// <p>Represents the response from the server after AWS Device Farm makes a request to install to a remote access session.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InstallToRemoteAccessSessionResult {
    /// <p>An app to upload or that has been uploaded.</p>
    #[serde(rename = "appUpload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_upload: Option<Upload>,
}

/// <p>Represents the instance profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InstanceProfile {
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description of the instance profile.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An array of strings specifying the list of app packages that should not be cleaned up from the device after a test run is over.</p> <p>The list of packages is only considered if you set <code>packageCleanup</code> to <code>true</code>.</p>
    #[serde(rename = "excludeAppPackagesFromCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_app_packages_from_cleanup: Option<Vec<String>>,
    /// <p>The name of the instance profile.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>When set to <code>true</code>, Device Farm will remove app packages after a test run. The default value is <code>false</code> for private devices.</p>
    #[serde(rename = "packageCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_cleanup: Option<bool>,
    /// <p>When set to <code>true</code>, Device Farm will reboot the instance after a test run. The default value is <code>true</code>.</p>
    #[serde(rename = "rebootAfterUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reboot_after_use: Option<bool>,
}

/// <p>There was an error with the update request, or you do not have sufficient permissions to update this VPC endpoint configuration.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InvalidOperationException {
    pub message: Option<String>,
}

/// <p>Represents a device.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Job {
    /// <p>The job's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The job's result counters.</p>
    #[serde(rename = "counters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<Counters>,
    /// <p>When the job was created.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The device (phone or tablet).</p>
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    /// <p>Represents the total (metered or unmetered) minutes used by the job.</p>
    #[serde(rename = "deviceMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_minutes: Option<DeviceMinutes>,
    /// <p>The Amazon Resource Name (ARN) of the instance.</p>
    #[serde(rename = "instanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    /// <p>A message about the job's result.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The job's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The job&#39;s result.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING: A pending condition.</p> </li> <li> <p>PASSED: A passing condition.</p> </li> <li> <p>WARNED: A warning condition.</p> </li> <li> <p>FAILED: A failed condition.</p> </li> <li> <p>SKIPPED: A skipped condition.</p> </li> <li> <p>ERRORED: An error condition.</p> </li> <li> <p>STOPPED: A stopped condition.</p> </li> </ul></p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>The job's start time.</p>
    #[serde(rename = "started")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<f64>,
    /// <p><p>The job&#39;s status.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING: A pending status.</p> </li> <li> <p>PENDING<em>CONCURRENCY: A pending concurrency status.</p> </li> <li> <p>PENDING</em>DEVICE: A pending device status.</p> </li> <li> <p>PROCESSING: A processing status.</p> </li> <li> <p>SCHEDULING: A scheduling status.</p> </li> <li> <p>PREPARING: A preparing status.</p> </li> <li> <p>RUNNING: A running status.</p> </li> <li> <p>COMPLETED: A completed status.</p> </li> <li> <p>STOPPING: A stopping status.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The job's stop time.</p>
    #[serde(rename = "stopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<f64>,
    /// <p><p>The job&#39;s type.</p> <p>Allowed values include the following:</p> <ul> <li> <p>BUILTIN<em>FUZZ: The built-in fuzz type.</p> </li> <li> <p>BUILTIN</em>EXPLORER: For Android, an app explorer that will traverse an Android app, interacting with it and capturing screenshots at the same time.</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT: The Appium Java JUnit type.</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG: The Appium Java TestNG type.</p> </li> <li> <p>APPIUM<em>PYTHON: The Appium Python type.</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT: The Appium Java JUnit type for Web apps.</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG: The Appium Java TestNG type for Web apps.</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON: The Appium Python type for Web apps.</p> </li> <li> <p>CALABASH: The Calabash type.</p> </li> <li> <p>INSTRUMENTATION: The Instrumentation type.</p> </li> <li> <p>UIAUTOMATION: The uiautomation type.</p> </li> <li> <p>UIAUTOMATOR: The uiautomator type.</p> </li> <li> <p>XCTEST: The XCode test type.</p> </li> <li> <p>XCTEST</em>UI: The XCode UI test type.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>This value is set to true if video capture is enabled; otherwise, it is set to false.</p>
    #[serde(rename = "videoCapture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_capture: Option<bool>,
    /// <p>The endpoint for streaming device video.</p>
    #[serde(rename = "videoEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_endpoint: Option<String>,
}

/// <p>A limit was exceeded.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LimitExceededException {
    /// <p>Any additional information about the exception.</p>
    pub message: Option<String>,
}

/// <p>Represents a request to the list artifacts operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListArtifactsRequest {
    /// <p>The Run, Job, Suite, or Test ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The artifacts&#39; type.</p> <p>Allowed values include:</p> <ul> <li> <p>FILE: The artifacts are files.</p> </li> <li> <p>LOG: The artifacts are logs.</p> </li> <li> <p>SCREENSHOT: The artifacts are screenshots.</p> </li> </ul></p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Represents the result of a list artifacts operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListArtifactsResult {
    /// <p>Information about the artifacts.</p>
    #[serde(rename = "artifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<Artifact>>,
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned, which can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDeviceInstancesRequest {
    /// <p>An integer specifying the maximum number of items you want to return in the API response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListDeviceInstancesResult {
    /// <p>An object containing information about your device instances.</p>
    #[serde(rename = "deviceInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_instances: Option<Vec<DeviceInstance>>,
    /// <p>An identifier that can be used in the next call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list device pools request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDevicePoolsRequest {
    /// <p>The project ARN.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The device pools&#39; type.</p> <p>Allowed values include:</p> <ul> <li> <p>CURATED: A device pool that is created and managed by AWS Device Farm.</p> </li> <li> <p>PRIVATE: A device pool that is created and managed by the device pool developer.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents the result of a list device pools request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListDevicePoolsResult {
    /// <p>Information about the device pools.</p>
    #[serde(rename = "devicePools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_pools: Option<Vec<DevicePool>>,
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned, which can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list devices request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDevicesRequest {
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list devices operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListDevicesResult {
    /// <p>Information about the devices.</p>
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned, which can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListInstanceProfilesRequest {
    /// <p>An integer specifying the maximum number of items you want to return in the API response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListInstanceProfilesResult {
    /// <p>An object containing information about your instance profiles.</p>
    #[serde(rename = "instanceProfiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profiles: Option<Vec<InstanceProfile>>,
    /// <p>An identifier that can be used in the next call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents a request to the list jobs operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListJobsRequest {
    /// <p>The run's Amazon Resource Name (ARN).</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list jobs request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListJobsResult {
    /// <p>Information about the jobs.</p>
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned, which can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListNetworkProfilesRequest {
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to list network profiles.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of network profile you wish to return information about. Valid values are listed below.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListNetworkProfilesResult {
    /// <p>A list of the available network profiles.</p>
    #[serde(rename = "networkProfiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profiles: Option<Vec<NetworkProfile>>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListOfferingPromotionsRequest {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListOfferingPromotionsResult {
    /// <p>An identifier to be used in the next call to this operation, to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the offering promotions.</p>
    #[serde(rename = "offeringPromotions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_promotions: Option<Vec<OfferingPromotion>>,
}

/// <p>Represents the request to list the offering transaction history.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListOfferingTransactionsRequest {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Returns the transaction log of the specified offerings.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListOfferingTransactionsResult {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The audit log of subscriptions you have purchased and modified through AWS Device Farm.</p>
    #[serde(rename = "offeringTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_transactions: Option<Vec<OfferingTransaction>>,
}

/// <p>Represents the request to list all offerings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListOfferingsRequest {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the return values of the list of offerings.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListOfferingsResult {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A value representing the list offering results.</p>
    #[serde(rename = "offerings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offerings: Option<Vec<Offering>>,
}

/// <p>Represents a request to the list projects operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListProjectsRequest {
    /// <p>Optional. If no Amazon Resource Name (ARN) is specified, then AWS Device Farm returns a list of all projects for the AWS account. You can also specify a project ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list projects request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListProjectsResult {
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned, which can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the projects.</p>
    #[serde(rename = "projects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<Project>>,
}

/// <p>Represents the request to return information about the remote access session.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRemoteAccessSessionsRequest {
    /// <p>The Amazon Resource Name (ARN) of the remote access session about which you are requesting information.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the response from the server after AWS Device Farm makes a request to return information about the remote access session.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListRemoteAccessSessionsResult {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A container representing the metadata from the service about each remote access session you are requesting.</p>
    #[serde(rename = "remoteAccessSessions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_sessions: Option<Vec<RemoteAccessSession>>,
}

/// <p>Represents a request to the list runs operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRunsRequest {
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to list runs.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list runs request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListRunsResult {
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned, which can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the runs.</p>
    #[serde(rename = "runs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs: Option<Vec<Run>>,
}

/// <p>Represents a request to the list samples operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSamplesRequest {
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to list samples.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list samples request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListSamplesResult {
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned, which can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the samples.</p>
    #[serde(rename = "samples")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samples: Option<Vec<Sample>>,
}

/// <p>Represents a request to the list suites operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSuitesRequest {
    /// <p>The job's Amazon Resource Name (ARN).</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list suites request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListSuitesResult {
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned, which can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the suites.</p>
    #[serde(rename = "suites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suites: Option<Vec<Suite>>,
}

/// <p>Represents a request to the list tests operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTestsRequest {
    /// <p>The test suite's Amazon Resource Name (ARN).</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list tests request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTestsResult {
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned, which can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the tests.</p>
    #[serde(rename = "tests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tests: Option<Vec<Test>>,
}

/// <p>Represents a request to the list unique problems operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUniqueProblemsRequest {
    /// <p>The unique problems' ARNs.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the result of a list unique problems request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListUniqueProblemsResult {
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned, which can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>Information about the unique problems.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING: A pending condition.</p> </li> <li> <p>PASSED: A passing condition.</p> </li> <li> <p>WARNED: A warning condition.</p> </li> <li> <p>FAILED: A failed condition.</p> </li> <li> <p>SKIPPED: A skipped condition.</p> </li> <li> <p>ERRORED: An error condition.</p> </li> <li> <p>STOPPED: A stopped condition.</p> </li> </ul></p>
    #[serde(rename = "uniqueProblems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_problems: Option<::std::collections::HashMap<String, Vec<UniqueProblem>>>,
}

/// <p>Represents a request to the list uploads operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUploadsRequest {
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to list uploads.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The type of upload.</p> <p>Must be one of the following values:</p> <ul> <li> <p>ANDROID<em>APP: An Android upload.</p> </li> <li> <p>IOS</em>APP: An iOS upload.</p> </li> <li> <p>WEB<em>APP: A web appliction upload.</p> </li> <li> <p>EXTERNAL</em>DATA: An external data upload.</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT<em>TEST</em>PACKAGE: An Appium Java JUnit test package upload.</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG<em>TEST</em>PACKAGE: An Appium Java TestNG test package upload.</p> </li> <li> <p>APPIUM<em>PYTHON</em>TEST<em>PACKAGE: An Appium Python test package upload.</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT<em>TEST</em>PACKAGE: An Appium Java JUnit test package upload.</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG</em>TEST<em>PACKAGE: An Appium Java TestNG test package upload.</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON</em>TEST<em>PACKAGE: An Appium Python test package upload.</p> </li> <li> <p>CALABASH</em>TEST<em>PACKAGE: A Calabash test package upload.</p> </li> <li> <p>INSTRUMENTATION</em>TEST<em>PACKAGE: An instrumentation upload.</p> </li> <li> <p>UIAUTOMATION</em>TEST<em>PACKAGE: A uiautomation test package upload.</p> </li> <li> <p>UIAUTOMATOR</em>TEST<em>PACKAGE: A uiautomator test package upload.</p> </li> <li> <p>XCTEST</em>TEST<em>PACKAGE: An XCode test package upload.</p> </li> <li> <p>XCTEST</em>UI<em>TEST</em>PACKAGE: An XCode UI test package upload.</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT<em>TEST</em>SPEC: An Appium Java JUnit test spec upload.</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG<em>TEST</em>SPEC: An Appium Java TestNG test spec upload.</p> </li> <li> <p>APPIUM<em>PYTHON</em>TEST<em>SPEC: An Appium Python test spec upload.</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT<em>TEST</em>SPEC: An Appium Java JUnit test spec upload.</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG</em>TEST<em>SPEC: An Appium Java TestNG test spec upload.</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON</em>TEST<em>SPEC: An Appium Python test spec upload.</p> </li> <li> <p>INSTRUMENTATION</em>TEST<em>SPEC: An instrumentation test spec upload.</p> </li> <li> <p>XCTEST</em>UI<em>TEST</em>SPEC: An XCode UI test spec upload.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents the result of a list uploads request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListUploadsResult {
    /// <p>If the number of items that are returned is significantly large, this is an identifier that is also returned, which can be used in a subsequent call to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the uploads.</p>
    #[serde(rename = "uploads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploads: Option<Vec<Upload>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListVPCEConfigurationsRequest {
    /// <p>An integer specifying the maximum number of items you want to return in the API response.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListVPCEConfigurationsResult {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of <code>VPCEConfiguration</code> objects containing information about your VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configurations: Option<Vec<VPCEConfiguration>>,
}

/// <p>Represents a latitude and longitude pair, expressed in geographic coordinate system degrees (for example 47.6204, -122.3491).</p> <p>Elevation is currently not supported.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// <p>The latitude.</p>
    #[serde(rename = "latitude")]
    pub latitude: f64,
    /// <p>The longitude.</p>
    #[serde(rename = "longitude")]
    pub longitude: f64,
}

/// <p>A number representing the monetary amount for an offering or transaction.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MonetaryAmount {
    /// <p>The numerical amount of an offering or transaction.</p>
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// <p>The currency code of a monetary amount. For example, <code>USD</code> means "U.S. dollars."</p>
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
}

/// <p>An array of settings that describes characteristics of a network profile.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct NetworkProfile {
    /// <p>The Amazon Resource Name (ARN) of the network profile.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The description of the network profile.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    #[serde(rename = "downlinkBandwidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_bandwidth_bits: Option<i64>,
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "downlinkDelayMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_delay_ms: Option<i64>,
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "downlinkJitterMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_jitter_ms: Option<i64>,
    /// <p>Proportion of received packets that fail to arrive from 0 to 100 percent.</p>
    #[serde(rename = "downlinkLossPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_loss_percent: Option<i64>,
    /// <p>The name of the network profile.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of network profile. Valid values are listed below.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    #[serde(rename = "uplinkBandwidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_bandwidth_bits: Option<i64>,
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "uplinkDelayMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_delay_ms: Option<i64>,
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "uplinkJitterMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_jitter_ms: Option<i64>,
    /// <p>Proportion of transmitted packets that fail to arrive from 0 to 100 percent.</p>
    #[serde(rename = "uplinkLossPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_loss_percent: Option<i64>,
}

/// <p>Exception gets thrown when a user is not eligible to perform the specified transaction.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NotEligibleException {
    /// <p>The HTTP response code of a Not Eligible exception.</p>
    pub message: Option<String>,
}

/// <p>The specified entity was not found.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NotFoundException {
    /// <p>Any additional information about the exception.</p>
    pub message: Option<String>,
}

/// <p>Represents the metadata of a device offering.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Offering {
    /// <p>A string describing the offering.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID that corresponds to a device offering.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The platform of the device (e.g., ANDROID or IOS).</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Specifies whether there are recurring charges for the offering.</p>
    #[serde(rename = "recurringCharges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charges: Option<Vec<RecurringCharge>>,
    /// <p>The type of offering (e.g., "RECURRING") for a device.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents information about an offering promotion.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OfferingPromotion {
    /// <p>A string describing the offering promotion.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the offering promotion.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>The status of the offering.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OfferingStatus {
    /// <p>The date on which the offering is effective.</p>
    #[serde(rename = "effectiveOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_on: Option<f64>,
    /// <p>Represents the metadata of an offering status.</p>
    #[serde(rename = "offering")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering: Option<Offering>,
    /// <p>The number of available devices in the offering.</p>
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// <p>The type specified for the offering status.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents the metadata of an offering transaction.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OfferingTransaction {
    /// <p>The cost of an offering transaction.</p>
    #[serde(rename = "cost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<MonetaryAmount>,
    /// <p>The date on which an offering transaction was created.</p>
    #[serde(rename = "createdOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    /// <p>The ID that corresponds to a device offering promotion.</p>
    #[serde(rename = "offeringPromotionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_promotion_id: Option<String>,
    /// <p>The status of an offering transaction.</p>
    #[serde(rename = "offeringStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_status: Option<OfferingStatus>,
    /// <p>The transaction ID of the offering transaction.</p>
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// <p>Represents a specific warning or failure.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Problem {
    /// <p>Information about the associated device.</p>
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    /// <p>Information about the associated job.</p>
    #[serde(rename = "job")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<ProblemDetail>,
    /// <p>A message about the problem's result.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p><p>The problem&#39;s result.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING: A pending condition.</p> </li> <li> <p>PASSED: A passing condition.</p> </li> <li> <p>WARNED: A warning condition.</p> </li> <li> <p>FAILED: A failed condition.</p> </li> <li> <p>SKIPPED: A skipped condition.</p> </li> <li> <p>ERRORED: An error condition.</p> </li> <li> <p>STOPPED: A stopped condition.</p> </li> </ul></p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>Information about the associated run.</p>
    #[serde(rename = "run")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<ProblemDetail>,
    /// <p>Information about the associated suite.</p>
    #[serde(rename = "suite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suite: Option<ProblemDetail>,
    /// <p>Information about the associated test.</p>
    #[serde(rename = "test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<ProblemDetail>,
}

/// <p>Information about a problem detail.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ProblemDetail {
    /// <p>The problem detail's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The problem detail's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Represents an operating-system neutral workspace for running and managing tests.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Project {
    /// <p>The project's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>When the project was created.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The default number of minutes (at the project level) a test run will execute before it times out. Default value is 60 minutes.</p>
    #[serde(rename = "defaultJobTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_job_timeout_minutes: Option<i64>,
    /// <p>The project's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Represents a request for a purchase offering.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PurchaseOfferingRequest {
    /// <p>The ID of the offering.</p>
    #[serde(rename = "offeringId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    /// <p>The ID of the offering promotion to be applied to the purchase.</p>
    #[serde(rename = "offeringPromotionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_promotion_id: Option<String>,
    /// <p>The number of device slots you wish to purchase in an offering request.</p>
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

/// <p>The result of the purchase offering (e.g., success or failure).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PurchaseOfferingResult {
    /// <p>Represents the offering transaction for the purchase result.</p>
    #[serde(rename = "offeringTransaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_transaction: Option<OfferingTransaction>,
}

/// <p>Represents the set of radios and their states on a device. Examples of radios include Wi-Fi, GPS, Bluetooth, and NFC.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Radios {
    /// <p>True if Bluetooth is enabled at the beginning of the test; otherwise, false.</p>
    #[serde(rename = "bluetooth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bluetooth: Option<bool>,
    /// <p>True if GPS is enabled at the beginning of the test; otherwise, false.</p>
    #[serde(rename = "gps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps: Option<bool>,
    /// <p>True if NFC is enabled at the beginning of the test; otherwise, false.</p>
    #[serde(rename = "nfc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfc: Option<bool>,
    /// <p>True if Wi-Fi is enabled at the beginning of the test; otherwise, false.</p>
    #[serde(rename = "wifi")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi: Option<bool>,
}

/// <p>Specifies whether charges for devices will be recurring.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RecurringCharge {
    /// <p>The cost of the recurring charge.</p>
    #[serde(rename = "cost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<MonetaryAmount>,
    /// <p>The frequency in which charges will recur.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
}

/// <p>Represents information about the remote access session.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RemoteAccessSession {
    /// <p>The Amazon Resource Name (ARN) of the remote access session.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The billing method of the remote access session. Possible values include <code>METERED</code> or <code>UNMETERED</code>. For more information about metered devices, see <a href="http://docs.aws.amazon.com/devicefarm/latest/developerguide/welcome.html#welcome-terminology">AWS Device Farm terminology</a>."</p>
    #[serde(rename = "billingMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_method: Option<String>,
    /// <p>Unique identifier of your client for the remote access session. Only returned if remote debugging is enabled for the remote access session.</p>
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The date and time the remote access session was created.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>The device (phone or tablet) used in the remote access session.</p>
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    /// <p>The number of minutes a device is used in a remote access sesssion (including setup and teardown minutes).</p>
    #[serde(rename = "deviceMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_minutes: Option<DeviceMinutes>,
    /// <p>Unique device identifier for the remote device. Only returned if remote debugging is enabled for the remote access session.</p>
    #[serde(rename = "deviceUdid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_udid: Option<String>,
    /// <p>The endpoint for the remote access sesssion.</p>
    #[serde(rename = "endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// <p>IP address of the EC2 host where you need to connect to remotely debug devices. Only returned if remote debugging is enabled for the remote access session.</p>
    #[serde(rename = "hostAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_address: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the instance.</p>
    #[serde(rename = "instanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    /// <p><p>The interaction mode of the remote access session. Valid values are:</p> <ul> <li> <p>INTERACTIVE: You can interact with the iOS device by viewing, touching, and rotating the screen. You <b>cannot</b> run XCUITest framework-based tests in this mode.</p> </li> <li> <p>NO<em>VIDEO: You are connected to the device but cannot interact with it or view the screen. This mode has the fastest test execution speed. You <b>can</b> run XCUITest framework-based tests in this mode.</p> </li> <li> <p>VIDEO</em>ONLY: You can view the screen but cannot touch or rotate it. You <b>can</b> run XCUITest framework-based tests and watch the screen in this mode.</p> </li> </ul></p>
    #[serde(rename = "interactionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_mode: Option<String>,
    /// <p>A message about the remote access session.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The name of the remote access session.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>This flag is set to <code>true</code> if remote debugging is enabled for the remote access session.</p>
    #[serde(rename = "remoteDebugEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_debug_enabled: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) for the app to be recorded in the remote access session.</p>
    #[serde(rename = "remoteRecordAppArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_record_app_arn: Option<String>,
    /// <p>This flag is set to <code>true</code> if remote recording is enabled for the remote access session.</p>
    #[serde(rename = "remoteRecordEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_record_enabled: Option<bool>,
    /// <p><p>The result of the remote access session. Can be any of the following:</p> <ul> <li> <p>PENDING: A pending condition.</p> </li> <li> <p>PASSED: A passing condition.</p> </li> <li> <p>WARNED: A warning condition.</p> </li> <li> <p>FAILED: A failed condition.</p> </li> <li> <p>SKIPPED: A skipped condition.</p> </li> <li> <p>ERRORED: An error condition.</p> </li> <li> <p>STOPPED: A stopped condition.</p> </li> </ul></p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>When set to <code>true</code>, for private devices, Device Farm will not sign your app again. For public devices, Device Farm always signs your apps again and this parameter has no effect.</p> <p>For more information about how Device Farm re-signs your app(s), see <a href="https://aws.amazon.com/device-farm/faq/">Do you modify my app?</a> in the <i>AWS Device Farm FAQs</i>.</p>
    #[serde(rename = "skipAppResign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_app_resign: Option<bool>,
    /// <p>The date and time the remote access session was started.</p>
    #[serde(rename = "started")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<f64>,
    /// <p><p>The status of the remote access session. Can be any of the following:</p> <ul> <li> <p>PENDING: A pending status.</p> </li> <li> <p>PENDING<em>CONCURRENCY: A pending concurrency status.</p> </li> <li> <p>PENDING</em>DEVICE: A pending device status.</p> </li> <li> <p>PROCESSING: A processing status.</p> </li> <li> <p>SCHEDULING: A scheduling status.</p> </li> <li> <p>PREPARING: A preparing status.</p> </li> <li> <p>RUNNING: A running status.</p> </li> <li> <p>COMPLETED: A completed status.</p> </li> <li> <p>STOPPING: A stopping status.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The date and time the remote access session was stopped.</p>
    #[serde(rename = "stopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<f64>,
}

/// <p>A request representing an offering renewal.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RenewOfferingRequest {
    /// <p>The ID of a request to renew an offering.</p>
    #[serde(rename = "offeringId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    /// <p>The quantity requested in an offering renewal.</p>
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

/// <p>The result of a renewal offering.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RenewOfferingResult {
    /// <p>Represents the status of the offering transaction for the renewal.</p>
    #[serde(rename = "offeringTransaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_transaction: Option<OfferingTransaction>,
}

/// <p>Represents the screen resolution of a device in height and width, expressed in pixels.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Resolution {
    /// <p>The screen resolution's height, expressed in pixels.</p>
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// <p>The screen resolution's width, expressed in pixels.</p>
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

/// <p>Represents a condition for a device pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    /// <p><p>The rule&#39;s stringified attribute. For example, specify the value as <code>&quot;&quot;abc&quot;&quot;</code>.</p> <p>Allowed values include:</p> <ul> <li> <p>ARN: The ARN.</p> </li> <li> <p>FORM<em>FACTOR: The form factor (for example, phone or tablet).</p> </li> <li> <p>MANUFACTURER: The manufacturer.</p> </li> <li> <p>PLATFORM: The platform (for example, Android or iOS).</p> </li> <li> <p>REMOTE</em>ACCESS<em>ENABLED: Whether the device is enabled for remote access.</p> </li> <li> <p>APPIUM</em>VERSION: The Appium version for the test.</p> </li> <li> <p>INSTANCE<em>ARN: The Amazon Resource Name (ARN) of the device instance.</p> </li> <li> <p>INSTANCE</em>LABELS: The label of the device instance.</p> </li> </ul></p>
    #[serde(rename = "attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    /// <p><p>The rule&#39;s operator.</p> <ul> <li> <p>EQUALS: The equals operator.</p> </li> <li> <p>GREATER<em>THAN: The greater-than operator.</p> </li> <li> <p>IN: The in operator.</p> </li> <li> <p>LESS</em>THAN: The less-than operator.</p> </li> <li> <p>NOT_IN: The not-in operator.</p> </li> <li> <p>CONTAINS: The contains operator.</p> </li> </ul></p>
    #[serde(rename = "operator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// <p>The rule's value.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Represents a test run on a set of devices with a given app package, test parameters, etc.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Run {
    /// <p>An app to upload or that has been uploaded.</p>
    #[serde(rename = "appUpload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_upload: Option<String>,
    /// <p>The run's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Specifies the billing method for a test run: <code>metered</code> or <code>unmetered</code>. If the parameter is not specified, the default value is <code>metered</code>.</p>
    #[serde(rename = "billingMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_method: Option<String>,
    /// <p>The total number of completed jobs.</p>
    #[serde(rename = "completedJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_jobs: Option<i64>,
    /// <p>The run's result counters.</p>
    #[serde(rename = "counters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<Counters>,
    /// <p>When the run was created.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>Output <code>CustomerArtifactPaths</code> object for the test run.</p>
    #[serde(rename = "customerArtifactPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_artifact_paths: Option<CustomerArtifactPaths>,
    /// <p>Represents the total (metered or unmetered) minutes used by the test run.</p>
    #[serde(rename = "deviceMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_minutes: Option<DeviceMinutes>,
    /// <p>The ARN of the device pool for the run.</p>
    #[serde(rename = "devicePoolArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_pool_arn: Option<String>,
    /// <p>For fuzz tests, this is the number of events, between 1 and 10000, that the UI fuzz test should perform.</p>
    #[serde(rename = "eventCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_count: Option<i64>,
    /// <p>The number of minutes the job will execute before it times out.</p>
    #[serde(rename = "jobTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_timeout_minutes: Option<i64>,
    /// <p>Information about the locale that is used for the run.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>Information about the location that is used for the run.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>A message about the run's result.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The run's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The network profile being used for a test run.</p>
    #[serde(rename = "networkProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    /// <p>Read-only URL for an object in S3 bucket where you can get the parsing results of the test package. If the test package doesn't parse, the reason why it doesn't parse appears in the file that this URL points to.</p>
    #[serde(rename = "parsingResultUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsing_result_url: Option<String>,
    /// <p><p>The run&#39;s platform.</p> <p>Allowed values include:</p> <ul> <li> <p>ANDROID: The Android platform.</p> </li> <li> <p>IOS: The iOS platform.</p> </li> </ul></p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Information about the radio states for the run.</p>
    #[serde(rename = "radios")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radios: Option<Radios>,
    /// <p><p>The run&#39;s result.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING: A pending condition.</p> </li> <li> <p>PASSED: A passing condition.</p> </li> <li> <p>WARNED: A warning condition.</p> </li> <li> <p>FAILED: A failed condition.</p> </li> <li> <p>SKIPPED: A skipped condition.</p> </li> <li> <p>ERRORED: An error condition.</p> </li> <li> <p>STOPPED: A stopped condition.</p> </li> </ul></p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>Supporting field for the result field. Set only if <code>result</code> is <code>SKIPPED</code>. <code>PARSING_FAILED</code> if the result is skipped because of test package parsing failure.</p>
    #[serde(rename = "resultCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<String>,
    /// <p>For fuzz tests, this is a seed to use for randomizing the UI fuzz test. Using the same seed value between tests ensures identical event sequences.</p>
    #[serde(rename = "seed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// <p>When set to <code>true</code>, for private devices, Device Farm will not sign your app again. For public devices, Device Farm always signs your apps again and this parameter has no effect.</p> <p>For more information about how Device Farm re-signs your app(s), see <a href="https://aws.amazon.com/device-farm/faq/">Do you modify my app?</a> in the <i>AWS Device Farm FAQs</i>.</p>
    #[serde(rename = "skipAppResign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_app_resign: Option<bool>,
    /// <p>The run's start time.</p>
    #[serde(rename = "started")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<f64>,
    /// <p><p>The run&#39;s status.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING: A pending status.</p> </li> <li> <p>PENDING<em>CONCURRENCY: A pending concurrency status.</p> </li> <li> <p>PENDING</em>DEVICE: A pending device status.</p> </li> <li> <p>PROCESSING: A processing status.</p> </li> <li> <p>SCHEDULING: A scheduling status.</p> </li> <li> <p>PREPARING: A preparing status.</p> </li> <li> <p>RUNNING: A running status.</p> </li> <li> <p>COMPLETED: A completed status.</p> </li> <li> <p>STOPPING: A stopping status.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The run's stop time.</p>
    #[serde(rename = "stopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<f64>,
    /// <p>The ARN of the YAML-formatted test specification for the run.</p>
    #[serde(rename = "testSpecArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_spec_arn: Option<String>,
    /// <p>The total number of jobs for the run.</p>
    #[serde(rename = "totalJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_jobs: Option<i64>,
    /// <p><p>The run&#39;s type.</p> <p>Must be one of the following values:</p> <ul> <li> <p>BUILTIN<em>FUZZ: The built-in fuzz type.</p> </li> <li> <p>BUILTIN</em>EXPLORER: For Android, an app explorer that will traverse an Android app, interacting with it and capturing screenshots at the same time.</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT: The Appium Java JUnit type.</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG: The Appium Java TestNG type.</p> </li> <li> <p>APPIUM<em>PYTHON: The Appium Python type.</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT: The Appium Java JUnit type for Web apps.</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG: The Appium Java TestNG type for Web apps.</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON: The Appium Python type for Web apps.</p> </li> <li> <p>CALABASH: The Calabash type.</p> </li> <li> <p>INSTRUMENTATION: The Instrumentation type.</p> </li> <li> <p>UIAUTOMATION: The uiautomation type.</p> </li> <li> <p>UIAUTOMATOR: The uiautomator type.</p> </li> <li> <p>XCTEST: The XCode test type.</p> </li> <li> <p>XCTEST</em>UI: The XCode UI test type.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The Device Farm console URL for the recording of the run.</p>
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}

/// <p>Represents a sample of performance data.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Sample {
    /// <p>The sample's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p><p>The sample&#39;s type.</p> <p>Must be one of the following values:</p> <ul> <li> <p>CPU: A CPU sample type. This is expressed as the app processing CPU time (including child processes) as reported by process, as a percentage.</p> </li> <li> <p>MEMORY: A memory usage sample type. This is expressed as the total proportional set size of an app process, in kilobytes.</p> </li> <li> <p>NATIVE<em>AVG</em>DRAWTIME</p> </li> <li> <p>NATIVE<em>FPS</p> </li> <li> <p>NATIVE</em>FRAMES</p> </li> <li> <p>NATIVE<em>MAX</em>DRAWTIME</p> </li> <li> <p>NATIVE<em>MIN</em>DRAWTIME</p> </li> <li> <p>OPENGL<em>AVG</em>DRAWTIME</p> </li> <li> <p>OPENGL<em>FPS</p> </li> <li> <p>OPENGL</em>FRAMES</p> </li> <li> <p>OPENGL<em>MAX</em>DRAWTIME</p> </li> <li> <p>OPENGL<em>MIN</em>DRAWTIME</p> </li> <li> <p>RX</p> </li> <li> <p>RX<em>RATE: The total number of bytes per second (TCP and UDP) that are sent, by app process.</p> </li> <li> <p>THREADS: A threads sample type. This is expressed as the total number of threads per app process.</p> </li> <li> <p>TX</p> </li> <li> <p>TX</em>RATE: The total number of bytes per second (TCP and UDP) that are received, by app process.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The pre-signed Amazon S3 URL that can be used with a corresponding GET request to download the sample's file.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Represents the settings for a run. Includes things like location, radio states, auxiliary apps, and network profiles.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ScheduleRunConfiguration {
    /// <p>A list of auxiliary apps for the run.</p>
    #[serde(rename = "auxiliaryApps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auxiliary_apps: Option<Vec<String>>,
    /// <p>Specifies the billing method for a test run: <code>metered</code> or <code>unmetered</code>. If the parameter is not specified, the default value is <code>metered</code>.</p>
    #[serde(rename = "billingMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_method: Option<String>,
    /// <p>Input <code>CustomerArtifactPaths</code> object for the scheduled run configuration.</p>
    #[serde(rename = "customerArtifactPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_artifact_paths: Option<CustomerArtifactPaths>,
    /// <p>The ARN of the extra data for the run. The extra data is a .zip file that AWS Device Farm will extract to external data for Android or the app's sandbox for iOS.</p>
    #[serde(rename = "extraDataPackageArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_data_package_arn: Option<String>,
    /// <p>Information about the locale that is used for the run.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>Information about the location that is used for the run.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// <p>Reserved for internal use.</p>
    #[serde(rename = "networkProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile_arn: Option<String>,
    /// <p>Information about the radio states for the run.</p>
    #[serde(rename = "radios")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radios: Option<Radios>,
    /// <p>An array of Amazon Resource Names (ARNs) for your VPC endpoint configurations.</p>
    #[serde(rename = "vpceConfigurationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration_arns: Option<Vec<String>>,
}

/// <p>Represents a request to the schedule run operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ScheduleRunRequest {
    /// <p>The ARN of the app to schedule a run.</p>
    #[serde(rename = "appArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    /// <p>Information about the settings for the run to be scheduled.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ScheduleRunConfiguration>,
    /// <p>The ARN of the device pool for the run to be scheduled.</p>
    #[serde(rename = "devicePoolArn")]
    pub device_pool_arn: String,
    /// <p>Specifies configuration information about a test run, such as the execution timeout (in minutes).</p>
    #[serde(rename = "executionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_configuration: Option<ExecutionConfiguration>,
    /// <p>The name for the run to be scheduled.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN of the project for the run to be scheduled.</p>
    #[serde(rename = "projectArn")]
    pub project_arn: String,
    /// <p>Information about the test for the run to be scheduled.</p>
    #[serde(rename = "test")]
    pub test: ScheduleRunTest,
}

/// <p>Represents the result of a schedule run request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ScheduleRunResult {
    /// <p>Information about the scheduled run.</p>
    #[serde(rename = "run")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<Run>,
}

/// <p>Represents additional test settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ScheduleRunTest {
    /// <p>The test's filter.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// <p><p>The test&#39;s parameters, such as the following test framework parameters and fixture settings:</p> <p>For Calabash tests:</p> <ul> <li> <p>profile: A cucumber profile, for example, &quot;my<em>profile</em>name&quot;.</p> </li> <li> <p>tags: You can limit execution to features or scenarios that have (or don&#39;t have) certain tags, for example, &quot;@smoke&quot; or &quot;@smoke,~@wip&quot;.</p> </li> </ul> <p>For Appium tests (all types):</p> <ul> <li> <p>appium<em>version: The Appium version. Currently supported values are &quot;1.4.16&quot;, &quot;1.6.3&quot;, &quot;latest&quot;, and &quot;default&quot;.</p> <ul> <li> <p>“latest” will run the latest Appium version supported by Device Farm (1.6.3).</p> </li> <li> <p>For “default”, Device Farm will choose a compatible version of Appium for the device. The current behavior is to run 1.4.16 on Android devices and iOS 9 and earlier, 1.6.3 for iOS 10 and later.</p> </li> <li> <p>This behavior is subject to change.</p> </li> </ul> </li> </ul> <p>For Fuzz tests (Android only):</p> <ul> <li> <p>event</em>count: The number of events, between 1 and 10000, that the UI fuzz test should perform.</p> </li> <li> <p>throttle: The time, in ms, between 0 and 1000, that the UI fuzz test should wait between events.</p> </li> <li> <p>seed: A seed to use for randomizing the UI fuzz test. Using the same seed value between tests ensures identical event sequences.</p> </li> </ul> <p>For Explorer tests:</p> <ul> <li> <p>username: A username to use if the Explorer encounters a login form. If not supplied, no username will be inserted.</p> </li> <li> <p>password: A password to use if the Explorer encounters a login form. If not supplied, no password will be inserted.</p> </li> </ul> <p>For Instrumentation:</p> <ul> <li> <p>filter: A test filter string. Examples:</p> <ul> <li> <p>Running a single test case: &quot;com.android.abc.Test1&quot;</p> </li> <li> <p>Running a single test: &quot;com.android.abc.Test1#smoke&quot;</p> </li> <li> <p>Running multiple tests: &quot;com.android.abc.Test1,com.android.abc.Test2&quot;</p> </li> </ul> </li> </ul> <p>For XCTest and XCTestUI:</p> <ul> <li> <p>filter: A test filter string. Examples:</p> <ul> <li> <p>Running a single test class: &quot;LoginTests&quot;</p> </li> <li> <p>Running a multiple test classes: &quot;LoginTests,SmokeTests&quot;</p> </li> <li> <p>Running a single test: &quot;LoginTests/testValid&quot;</p> </li> <li> <p>Running multiple tests: &quot;LoginTests/testValid,LoginTests/testInvalid&quot;</p> </li> </ul> </li> </ul> <p>For UIAutomator:</p> <ul> <li> <p>filter: A test filter string. Examples:</p> <ul> <li> <p>Running a single test case: &quot;com.android.abc.Test1&quot;</p> </li> <li> <p>Running a single test: &quot;com.android.abc.Test1#smoke&quot;</p> </li> <li> <p>Running multiple tests: &quot;com.android.abc.Test1,com.android.abc.Test2&quot;</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The ARN of the uploaded test that will be run.</p>
    #[serde(rename = "testPackageArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_package_arn: Option<String>,
    /// <p>The ARN of the YAML-formatted test specification.</p>
    #[serde(rename = "testSpecArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_spec_arn: Option<String>,
    /// <p><p>The test&#39;s type.</p> <p>Must be one of the following values:</p> <ul> <li> <p>BUILTIN<em>FUZZ: The built-in fuzz type.</p> </li> <li> <p>BUILTIN</em>EXPLORER: For Android, an app explorer that will traverse an Android app, interacting with it and capturing screenshots at the same time.</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT: The Appium Java JUnit type.</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG: The Appium Java TestNG type.</p> </li> <li> <p>APPIUM<em>PYTHON: The Appium Python type.</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT: The Appium Java JUnit type for Web apps.</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG: The Appium Java TestNG type for Web apps.</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON: The Appium Python type for Web apps.</p> </li> <li> <p>CALABASH: The Calabash type.</p> </li> <li> <p>INSTRUMENTATION: The Instrumentation type.</p> </li> <li> <p>UIAUTOMATION: The uiautomation type.</p> </li> <li> <p>UIAUTOMATOR: The uiautomator type.</p> </li> <li> <p>XCTEST: The XCode test type.</p> </li> <li> <p>XCTEST</em>UI: The XCode UI test type.</p> </li> </ul></p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>There was a problem with the service account.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ServiceAccountException {
    /// <p>Any additional information about the exception.</p>
    pub message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopJobRequest {
    /// <p>Represents the Amazon Resource Name (ARN) of the Device Farm job you wish to stop.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StopJobResult {
    /// <p>The job that was stopped.</p>
    #[serde(rename = "job")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

/// <p>Represents the request to stop the remote access session.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopRemoteAccessSessionRequest {
    /// <p>The Amazon Resource Name (ARN) of the remote access session you wish to stop.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the response from the server that describes the remote access session when AWS Device Farm stops the session.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StopRemoteAccessSessionResult {
    /// <p>A container representing the metadata from the service about the remote access session you are stopping.</p>
    #[serde(rename = "remoteAccessSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_session: Option<RemoteAccessSession>,
}

/// <p>Represents the request to stop a specific run.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopRunRequest {
    /// <p>Represents the Amazon Resource Name (ARN) of the Device Farm run you wish to stop.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

/// <p>Represents the results of your stop run attempt.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StopRunResult {
    /// <p>The run that was stopped.</p>
    #[serde(rename = "run")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<Run>,
}

/// <p>Represents a collection of one or more tests.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Suite {
    /// <p>The suite's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The suite's result counters.</p>
    #[serde(rename = "counters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<Counters>,
    /// <p>When the suite was created.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>Represents the total (metered or unmetered) minutes used by the test suite.</p>
    #[serde(rename = "deviceMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_minutes: Option<DeviceMinutes>,
    /// <p>A message about the suite's result.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The suite's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The suite&#39;s result.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING: A pending condition.</p> </li> <li> <p>PASSED: A passing condition.</p> </li> <li> <p>WARNED: A warning condition.</p> </li> <li> <p>FAILED: A failed condition.</p> </li> <li> <p>SKIPPED: A skipped condition.</p> </li> <li> <p>ERRORED: An error condition.</p> </li> <li> <p>STOPPED: A stopped condition.</p> </li> </ul></p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>The suite's start time.</p>
    #[serde(rename = "started")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<f64>,
    /// <p><p>The suite&#39;s status.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING: A pending status.</p> </li> <li> <p>PENDING<em>CONCURRENCY: A pending concurrency status.</p> </li> <li> <p>PENDING</em>DEVICE: A pending device status.</p> </li> <li> <p>PROCESSING: A processing status.</p> </li> <li> <p>SCHEDULING: A scheduling status.</p> </li> <li> <p>PREPARING: A preparing status.</p> </li> <li> <p>RUNNING: A running status.</p> </li> <li> <p>COMPLETED: A completed status.</p> </li> <li> <p>STOPPING: A stopping status.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The suite's stop time.</p>
    #[serde(rename = "stopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<f64>,
    /// <p><p>The suite&#39;s type.</p> <p>Must be one of the following values:</p> <ul> <li> <p>BUILTIN<em>FUZZ: The built-in fuzz type.</p> </li> <li> <p>BUILTIN</em>EXPLORER: For Android, an app explorer that will traverse an Android app, interacting with it and capturing screenshots at the same time.</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT: The Appium Java JUnit type.</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG: The Appium Java TestNG type.</p> </li> <li> <p>APPIUM<em>PYTHON: The Appium Python type.</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT: The Appium Java JUnit type for Web apps.</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG: The Appium Java TestNG type for Web apps.</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON: The Appium Python type for Web apps.</p> </li> <li> <p>CALABASH: The Calabash type.</p> </li> <li> <p>INSTRUMENTATION: The Instrumentation type.</p> </li> <li> <p>UIAUTOMATION: The uiautomation type.</p> </li> <li> <p>UIAUTOMATOR: The uiautomator type.</p> </li> <li> <p>XCTEST: The XCode test type.</p> </li> <li> <p>XCTEST</em>UI: The XCode UI test type.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents a condition that is evaluated.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Test {
    /// <p>The test's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The test's result counters.</p>
    #[serde(rename = "counters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<Counters>,
    /// <p>When the test was created.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>Represents the total (metered or unmetered) minutes used by the test.</p>
    #[serde(rename = "deviceMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_minutes: Option<DeviceMinutes>,
    /// <p>A message about the test's result.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The test's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The test&#39;s result.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING: A pending condition.</p> </li> <li> <p>PASSED: A passing condition.</p> </li> <li> <p>WARNED: A warning condition.</p> </li> <li> <p>FAILED: A failed condition.</p> </li> <li> <p>SKIPPED: A skipped condition.</p> </li> <li> <p>ERRORED: An error condition.</p> </li> <li> <p>STOPPED: A stopped condition.</p> </li> </ul></p>
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// <p>The test's start time.</p>
    #[serde(rename = "started")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<f64>,
    /// <p><p>The test&#39;s status.</p> <p>Allowed values include:</p> <ul> <li> <p>PENDING: A pending status.</p> </li> <li> <p>PENDING<em>CONCURRENCY: A pending concurrency status.</p> </li> <li> <p>PENDING</em>DEVICE: A pending device status.</p> </li> <li> <p>PROCESSING: A processing status.</p> </li> <li> <p>SCHEDULING: A scheduling status.</p> </li> <li> <p>PREPARING: A preparing status.</p> </li> <li> <p>RUNNING: A running status.</p> </li> <li> <p>COMPLETED: A completed status.</p> </li> <li> <p>STOPPING: A stopping status.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The test's stop time.</p>
    #[serde(rename = "stopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<f64>,
    /// <p><p>The test&#39;s type.</p> <p>Must be one of the following values:</p> <ul> <li> <p>BUILTIN<em>FUZZ: The built-in fuzz type.</p> </li> <li> <p>BUILTIN</em>EXPLORER: For Android, an app explorer that will traverse an Android app, interacting with it and capturing screenshots at the same time.</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT: The Appium Java JUnit type.</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG: The Appium Java TestNG type.</p> </li> <li> <p>APPIUM<em>PYTHON: The Appium Python type.</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT: The Appium Java JUnit type for Web apps.</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG: The Appium Java TestNG type for Web apps.</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON: The Appium Python type for Web apps.</p> </li> <li> <p>CALABASH: The Calabash type.</p> </li> <li> <p>INSTRUMENTATION: The Instrumentation type.</p> </li> <li> <p>UIAUTOMATION: The uiautomation type.</p> </li> <li> <p>UIAUTOMATOR: The uiautomator type.</p> </li> <li> <p>XCTEST: The XCode test type.</p> </li> <li> <p>XCTEST</em>UI: The XCode UI test type.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Represents information about free trial device minutes for an AWS account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TrialMinutes {
    /// <p>The number of free trial minutes remaining in the account.</p>
    #[serde(rename = "remaining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<f64>,
    /// <p>The total number of free trial minutes that the account started with.</p>
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
}

/// <p>A collection of one or more problems, grouped by their result.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UniqueProblem {
    /// <p>A message about the unique problems' result.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>Information about the problems.</p>
    #[serde(rename = "problems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problems: Option<Vec<Problem>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDeviceInstanceRequest {
    /// <p>The Amazon Resource Name (ARN) of the device instance.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>An array of strings that you want to associate with the device instance.</p>
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the profile that you want to associate with the device instance.</p>
    #[serde(rename = "profileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateDeviceInstanceResult {
    /// <p>An object containing information about your device instance.</p>
    #[serde(rename = "deviceInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_instance: Option<DeviceInstance>,
}

/// <p>Represents a request to the update device pool operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDevicePoolRequest {
    /// <p>The Amazon Resourc Name (ARN) of the Device Farm device pool you wish to update.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>A description of the device pool you wish to update.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A string representing the name of the device pool you wish to update.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Represents the rules you wish to modify for the device pool. Updating rules is optional; however, if you choose to update rules for your request, the update will replace the existing rules.</p>
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
}

/// <p>Represents the result of an update device pool request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateDevicePoolResult {
    /// <p>The device pool you just updated.</p>
    #[serde(rename = "devicePool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_pool: Option<DevicePool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateInstanceProfileRequest {
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The updated description for your instance profile.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An array of strings specifying the list of app packages that should not be cleaned up from the device after a test run is over.</p> <p>The list of packages is only considered if you set <code>packageCleanup</code> to <code>true</code>.</p>
    #[serde(rename = "excludeAppPackagesFromCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_app_packages_from_cleanup: Option<Vec<String>>,
    /// <p>The updated name for your instance profile.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The updated choice for whether you want to specify package cleanup. The default value is <code>false</code> for private devices.</p>
    #[serde(rename = "packageCleanup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_cleanup: Option<bool>,
    /// <p>The updated choice for whether you want to reboot the device after use. The default value is <code>true</code>.</p>
    #[serde(rename = "rebootAfterUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reboot_after_use: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateInstanceProfileResult {
    /// <p>An object containing information about your instance profile.</p>
    #[serde(rename = "instanceProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile: Option<InstanceProfile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateNetworkProfileRequest {
    /// <p>The Amazon Resource Name (ARN) of the project for which you want to update network profile settings.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The descriptoin of the network profile about which you are returning information.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    #[serde(rename = "downlinkBandwidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_bandwidth_bits: Option<i64>,
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "downlinkDelayMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_delay_ms: Option<i64>,
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "downlinkJitterMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_jitter_ms: Option<i64>,
    /// <p>Proportion of received packets that fail to arrive from 0 to 100 percent.</p>
    #[serde(rename = "downlinkLossPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downlink_loss_percent: Option<i64>,
    /// <p>The name of the network profile about which you are returning information.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of network profile you wish to return information about. Valid values are listed below.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The data throughput rate in bits per second, as an integer from 0 to 104857600.</p>
    #[serde(rename = "uplinkBandwidthBits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_bandwidth_bits: Option<i64>,
    /// <p>Delay time for all packets to destination in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "uplinkDelayMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_delay_ms: Option<i64>,
    /// <p>Time variation in the delay of received packets in milliseconds as an integer from 0 to 2000.</p>
    #[serde(rename = "uplinkJitterMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_jitter_ms: Option<i64>,
    /// <p>Proportion of transmitted packets that fail to arrive from 0 to 100 percent.</p>
    #[serde(rename = "uplinkLossPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_loss_percent: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateNetworkProfileResult {
    /// <p>A list of the available network profiles.</p>
    #[serde(rename = "networkProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
}

/// <p>Represents a request to the update project operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateProjectRequest {
    /// <p>The Amazon Resource Name (ARN) of the project whose name you wish to update.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The number of minutes a test run in the project will execute before it times out.</p>
    #[serde(rename = "defaultJobTimeoutMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_job_timeout_minutes: Option<i64>,
    /// <p>A string representing the new name of the project that you are updating.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Represents the result of an update project request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateProjectResult {
    /// <p>The project you wish to update.</p>
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUploadRequest {
    /// <p>The Amazon Resource Name (ARN) of the uploaded test spec.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The upload's content type (for example, "application/x-yaml").</p>
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>Set to true if the YAML file has changed and needs to be updated; otherwise, set to false.</p>
    #[serde(rename = "editContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_content: Option<bool>,
    /// <p>The upload's test spec file name. The name should not contain the '/' character. The test spec file name must end with the <code>.yaml</code> or <code>.yml</code> file extension.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateUploadResult {
    /// <p>A test spec uploaded to Device Farm.</p>
    #[serde(rename = "upload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<Upload>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateVPCEConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the VPC endpoint configuration you want to update.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The DNS (domain) name used to connect to your private service in your Amazon VPC. The DNS name must not already be in use on the Internet.</p>
    #[serde(rename = "serviceDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_dns_name: Option<String>,
    /// <p>An optional description, providing more details about your VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration_description: Option<String>,
    /// <p>The friendly name you give to your VPC endpoint configuration, to manage your configurations more easily.</p>
    #[serde(rename = "vpceConfigurationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration_name: Option<String>,
    /// <p>The name of the VPC endpoint service running inside your AWS account that you want Device Farm to test.</p>
    #[serde(rename = "vpceServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_service_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateVPCEConfigurationResult {
    /// <p>An object containing information about your VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration: Option<VPCEConfiguration>,
}

/// <p>An app or a set of one or more tests to upload or that have been uploaded.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Upload {
    /// <p>The upload's ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p><p>The upload&#39;s category. Allowed values include:</p> <ul> <li> <p>CURATED: An upload managed by AWS Device Farm.</p> </li> <li> <p>PRIVATE: An upload managed by the AWS Device Farm customer.</p> </li> </ul></p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The upload's content type (for example, "application/octet-stream").</p>
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>When the upload was created.</p>
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    /// <p>A message about the upload's result.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The upload's metadata. For example, for Android, this contains information that is parsed from the manifest and is displayed in the AWS Device Farm console after the associated app is uploaded.</p>
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// <p>The upload's file name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The upload&#39;s status.</p> <p>Must be one of the following values:</p> <ul> <li> <p>FAILED: A failed status.</p> </li> <li> <p>INITIALIZED: An initialized status.</p> </li> <li> <p>PROCESSING: A processing status.</p> </li> <li> <p>SUCCEEDED: A succeeded status.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>The upload&#39;s type.</p> <p>Must be one of the following values:</p> <ul> <li> <p>ANDROID<em>APP: An Android upload.</p> </li> <li> <p>IOS</em>APP: An iOS upload.</p> </li> <li> <p>WEB<em>APP: A web appliction upload.</p> </li> <li> <p>EXTERNAL</em>DATA: An external data upload.</p> </li> <li> <p>APPIUM<em>JAVA</em>JUNIT<em>TEST</em>PACKAGE: An Appium Java JUnit test package upload.</p> </li> <li> <p>APPIUM<em>JAVA</em>TESTNG<em>TEST</em>PACKAGE: An Appium Java TestNG test package upload.</p> </li> <li> <p>APPIUM<em>PYTHON</em>TEST<em>PACKAGE: An Appium Python test package upload.</p> </li> <li> <p>APPIUM</em>WEB<em>JAVA</em>JUNIT<em>TEST</em>PACKAGE: An Appium Java JUnit test package upload.</p> </li> <li> <p>APPIUM<em>WEB</em>JAVA<em>TESTNG</em>TEST<em>PACKAGE: An Appium Java TestNG test package upload.</p> </li> <li> <p>APPIUM</em>WEB<em>PYTHON</em>TEST<em>PACKAGE: An Appium Python test package upload.</p> </li> <li> <p>CALABASH</em>TEST<em>PACKAGE: A Calabash test package upload.</p> </li> <li> <p>INSTRUMENTATION</em>TEST<em>PACKAGE: An instrumentation upload.</p> </li> <li> <p>UIAUTOMATION</em>TEST<em>PACKAGE: A uiautomation test package upload.</p> </li> <li> <p>UIAUTOMATOR</em>TEST<em>PACKAGE: A uiautomator test package upload.</p> </li> <li> <p>XCTEST</em>TEST<em>PACKAGE: An XCode test package upload.</p> </li> <li> <p>XCTEST</em>UI<em>TEST</em>PACKAGE: An XCode UI test package upload.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The pre-signed Amazon S3 URL that was used to store a file through a corresponding PUT request.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Represents an Amazon Virtual Private Cloud (VPC) endpoint configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct VPCEConfiguration {
    /// <p>The Amazon Resource Name (ARN) of the VPC endpoint configuration.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The DNS name that maps to the private IP address of the service you want to access.</p>
    #[serde(rename = "serviceDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_dns_name: Option<String>,
    /// <p>An optional description, providing more details about your VPC endpoint configuration.</p>
    #[serde(rename = "vpceConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration_description: Option<String>,
    /// <p>The friendly name you give to your VPC endpoint configuration, to manage your configurations more easily.</p>
    #[serde(rename = "vpceConfigurationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_configuration_name: Option<String>,
    /// <p>The name of the VPC endpoint service running inside your AWS account that you want Device Farm to test.</p>
    #[serde(rename = "vpceServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpce_service_name: Option<String>,
}

/// Errors returned by CreateDevicePool
#[derive(Debug, PartialEq)]
pub enum CreateDevicePoolError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateDevicePoolError {
    pub fn from_body(body: &str) -> CreateDevicePoolError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        CreateDevicePoolError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateDevicePoolError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateDevicePoolError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        CreateDevicePoolError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDevicePoolError::Validation(error_message.to_string())
                    }
                    _ => CreateDevicePoolError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDevicePoolError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDevicePoolError {
    fn from(err: serde_json::error::Error) -> CreateDevicePoolError {
        CreateDevicePoolError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDevicePoolError {
    fn from(err: CredentialsError) -> CreateDevicePoolError {
        CreateDevicePoolError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDevicePoolError {
    fn from(err: HttpDispatchError) -> CreateDevicePoolError {
        CreateDevicePoolError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDevicePoolError {
    fn from(err: io::Error) -> CreateDevicePoolError {
        CreateDevicePoolError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDevicePoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDevicePoolError {
    fn description(&self) -> &str {
        match *self {
            CreateDevicePoolError::Argument(ref cause) => cause,
            CreateDevicePoolError::LimitExceeded(ref cause) => cause,
            CreateDevicePoolError::NotFound(ref cause) => cause,
            CreateDevicePoolError::ServiceAccount(ref cause) => cause,
            CreateDevicePoolError::Validation(ref cause) => cause,
            CreateDevicePoolError::Credentials(ref err) => err.description(),
            CreateDevicePoolError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDevicePoolError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateInstanceProfile
#[derive(Debug, PartialEq)]
pub enum CreateInstanceProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateInstanceProfileError {
    pub fn from_body(body: &str) -> CreateInstanceProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        CreateInstanceProfileError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateInstanceProfileError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateInstanceProfileError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        CreateInstanceProfileError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateInstanceProfileError::Validation(error_message.to_string())
                    }
                    _ => CreateInstanceProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateInstanceProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateInstanceProfileError {
    fn from(err: serde_json::error::Error) -> CreateInstanceProfileError {
        CreateInstanceProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateInstanceProfileError {
    fn from(err: CredentialsError) -> CreateInstanceProfileError {
        CreateInstanceProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateInstanceProfileError {
    fn from(err: HttpDispatchError) -> CreateInstanceProfileError {
        CreateInstanceProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateInstanceProfileError {
    fn from(err: io::Error) -> CreateInstanceProfileError {
        CreateInstanceProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateInstanceProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateInstanceProfileError {
    fn description(&self) -> &str {
        match *self {
            CreateInstanceProfileError::Argument(ref cause) => cause,
            CreateInstanceProfileError::LimitExceeded(ref cause) => cause,
            CreateInstanceProfileError::NotFound(ref cause) => cause,
            CreateInstanceProfileError::ServiceAccount(ref cause) => cause,
            CreateInstanceProfileError::Validation(ref cause) => cause,
            CreateInstanceProfileError::Credentials(ref err) => err.description(),
            CreateInstanceProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateInstanceProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateNetworkProfile
#[derive(Debug, PartialEq)]
pub enum CreateNetworkProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateNetworkProfileError {
    pub fn from_body(body: &str) -> CreateNetworkProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        CreateNetworkProfileError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateNetworkProfileError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateNetworkProfileError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        CreateNetworkProfileError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateNetworkProfileError::Validation(error_message.to_string())
                    }
                    _ => CreateNetworkProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateNetworkProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateNetworkProfileError {
    fn from(err: serde_json::error::Error) -> CreateNetworkProfileError {
        CreateNetworkProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateNetworkProfileError {
    fn from(err: CredentialsError) -> CreateNetworkProfileError {
        CreateNetworkProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateNetworkProfileError {
    fn from(err: HttpDispatchError) -> CreateNetworkProfileError {
        CreateNetworkProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateNetworkProfileError {
    fn from(err: io::Error) -> CreateNetworkProfileError {
        CreateNetworkProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateNetworkProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateNetworkProfileError {
    fn description(&self) -> &str {
        match *self {
            CreateNetworkProfileError::Argument(ref cause) => cause,
            CreateNetworkProfileError::LimitExceeded(ref cause) => cause,
            CreateNetworkProfileError::NotFound(ref cause) => cause,
            CreateNetworkProfileError::ServiceAccount(ref cause) => cause,
            CreateNetworkProfileError::Validation(ref cause) => cause,
            CreateNetworkProfileError::Credentials(ref err) => err.description(),
            CreateNetworkProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateNetworkProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateProject
#[derive(Debug, PartialEq)]
pub enum CreateProjectError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateProjectError {
    pub fn from_body(body: &str) -> CreateProjectError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        CreateProjectError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateProjectError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateProjectError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        CreateProjectError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateProjectError::Validation(error_message.to_string())
                    }
                    _ => CreateProjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateProjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateProjectError {
    fn from(err: serde_json::error::Error) -> CreateProjectError {
        CreateProjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateProjectError {
    fn from(err: CredentialsError) -> CreateProjectError {
        CreateProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateProjectError {
    fn from(err: HttpDispatchError) -> CreateProjectError {
        CreateProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateProjectError {
    fn from(err: io::Error) -> CreateProjectError {
        CreateProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateProjectError {
    fn description(&self) -> &str {
        match *self {
            CreateProjectError::Argument(ref cause) => cause,
            CreateProjectError::LimitExceeded(ref cause) => cause,
            CreateProjectError::NotFound(ref cause) => cause,
            CreateProjectError::ServiceAccount(ref cause) => cause,
            CreateProjectError::Validation(ref cause) => cause,
            CreateProjectError::Credentials(ref err) => err.description(),
            CreateProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateProjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRemoteAccessSession
#[derive(Debug, PartialEq)]
pub enum CreateRemoteAccessSessionError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateRemoteAccessSessionError {
    pub fn from_body(body: &str) -> CreateRemoteAccessSessionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        CreateRemoteAccessSessionError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateRemoteAccessSessionError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        CreateRemoteAccessSessionError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        CreateRemoteAccessSessionError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateRemoteAccessSessionError::Validation(error_message.to_string())
                    }
                    _ => CreateRemoteAccessSessionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateRemoteAccessSessionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateRemoteAccessSessionError {
    fn from(err: serde_json::error::Error) -> CreateRemoteAccessSessionError {
        CreateRemoteAccessSessionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateRemoteAccessSessionError {
    fn from(err: CredentialsError) -> CreateRemoteAccessSessionError {
        CreateRemoteAccessSessionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateRemoteAccessSessionError {
    fn from(err: HttpDispatchError) -> CreateRemoteAccessSessionError {
        CreateRemoteAccessSessionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateRemoteAccessSessionError {
    fn from(err: io::Error) -> CreateRemoteAccessSessionError {
        CreateRemoteAccessSessionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateRemoteAccessSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRemoteAccessSessionError {
    fn description(&self) -> &str {
        match *self {
            CreateRemoteAccessSessionError::Argument(ref cause) => cause,
            CreateRemoteAccessSessionError::LimitExceeded(ref cause) => cause,
            CreateRemoteAccessSessionError::NotFound(ref cause) => cause,
            CreateRemoteAccessSessionError::ServiceAccount(ref cause) => cause,
            CreateRemoteAccessSessionError::Validation(ref cause) => cause,
            CreateRemoteAccessSessionError::Credentials(ref err) => err.description(),
            CreateRemoteAccessSessionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateRemoteAccessSessionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUpload
#[derive(Debug, PartialEq)]
pub enum CreateUploadError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateUploadError {
    pub fn from_body(body: &str) -> CreateUploadError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => CreateUploadError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        CreateUploadError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => CreateUploadError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        CreateUploadError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateUploadError::Validation(error_message.to_string())
                    }
                    _ => CreateUploadError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateUploadError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateUploadError {
    fn from(err: serde_json::error::Error) -> CreateUploadError {
        CreateUploadError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateUploadError {
    fn from(err: CredentialsError) -> CreateUploadError {
        CreateUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateUploadError {
    fn from(err: HttpDispatchError) -> CreateUploadError {
        CreateUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateUploadError {
    fn from(err: io::Error) -> CreateUploadError {
        CreateUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUploadError {
    fn description(&self) -> &str {
        match *self {
            CreateUploadError::Argument(ref cause) => cause,
            CreateUploadError::LimitExceeded(ref cause) => cause,
            CreateUploadError::NotFound(ref cause) => cause,
            CreateUploadError::ServiceAccount(ref cause) => cause,
            CreateUploadError::Validation(ref cause) => cause,
            CreateUploadError::Credentials(ref err) => err.description(),
            CreateUploadError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateVPCEConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateVPCEConfigurationError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateVPCEConfigurationError {
    pub fn from_body(body: &str) -> CreateVPCEConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        CreateVPCEConfigurationError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateVPCEConfigurationError::LimitExceeded(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        CreateVPCEConfigurationError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateVPCEConfigurationError::Validation(error_message.to_string())
                    }
                    _ => CreateVPCEConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateVPCEConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateVPCEConfigurationError {
    fn from(err: serde_json::error::Error) -> CreateVPCEConfigurationError {
        CreateVPCEConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateVPCEConfigurationError {
    fn from(err: CredentialsError) -> CreateVPCEConfigurationError {
        CreateVPCEConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateVPCEConfigurationError {
    fn from(err: HttpDispatchError) -> CreateVPCEConfigurationError {
        CreateVPCEConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateVPCEConfigurationError {
    fn from(err: io::Error) -> CreateVPCEConfigurationError {
        CreateVPCEConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateVPCEConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateVPCEConfigurationError {
    fn description(&self) -> &str {
        match *self {
            CreateVPCEConfigurationError::Argument(ref cause) => cause,
            CreateVPCEConfigurationError::LimitExceeded(ref cause) => cause,
            CreateVPCEConfigurationError::ServiceAccount(ref cause) => cause,
            CreateVPCEConfigurationError::Validation(ref cause) => cause,
            CreateVPCEConfigurationError::Credentials(ref err) => err.description(),
            CreateVPCEConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateVPCEConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDevicePool
#[derive(Debug, PartialEq)]
pub enum DeleteDevicePoolError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteDevicePoolError {
    pub fn from_body(body: &str) -> DeleteDevicePoolError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        DeleteDevicePoolError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DeleteDevicePoolError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteDevicePoolError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        DeleteDevicePoolError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDevicePoolError::Validation(error_message.to_string())
                    }
                    _ => DeleteDevicePoolError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDevicePoolError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDevicePoolError {
    fn from(err: serde_json::error::Error) -> DeleteDevicePoolError {
        DeleteDevicePoolError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDevicePoolError {
    fn from(err: CredentialsError) -> DeleteDevicePoolError {
        DeleteDevicePoolError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDevicePoolError {
    fn from(err: HttpDispatchError) -> DeleteDevicePoolError {
        DeleteDevicePoolError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDevicePoolError {
    fn from(err: io::Error) -> DeleteDevicePoolError {
        DeleteDevicePoolError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDevicePoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDevicePoolError {
    fn description(&self) -> &str {
        match *self {
            DeleteDevicePoolError::Argument(ref cause) => cause,
            DeleteDevicePoolError::LimitExceeded(ref cause) => cause,
            DeleteDevicePoolError::NotFound(ref cause) => cause,
            DeleteDevicePoolError::ServiceAccount(ref cause) => cause,
            DeleteDevicePoolError::Validation(ref cause) => cause,
            DeleteDevicePoolError::Credentials(ref err) => err.description(),
            DeleteDevicePoolError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDevicePoolError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteInstanceProfile
#[derive(Debug, PartialEq)]
pub enum DeleteInstanceProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteInstanceProfileError {
    pub fn from_body(body: &str) -> DeleteInstanceProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        DeleteInstanceProfileError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DeleteInstanceProfileError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteInstanceProfileError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        DeleteInstanceProfileError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteInstanceProfileError::Validation(error_message.to_string())
                    }
                    _ => DeleteInstanceProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteInstanceProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteInstanceProfileError {
    fn from(err: serde_json::error::Error) -> DeleteInstanceProfileError {
        DeleteInstanceProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteInstanceProfileError {
    fn from(err: CredentialsError) -> DeleteInstanceProfileError {
        DeleteInstanceProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteInstanceProfileError {
    fn from(err: HttpDispatchError) -> DeleteInstanceProfileError {
        DeleteInstanceProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteInstanceProfileError {
    fn from(err: io::Error) -> DeleteInstanceProfileError {
        DeleteInstanceProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteInstanceProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteInstanceProfileError {
    fn description(&self) -> &str {
        match *self {
            DeleteInstanceProfileError::Argument(ref cause) => cause,
            DeleteInstanceProfileError::LimitExceeded(ref cause) => cause,
            DeleteInstanceProfileError::NotFound(ref cause) => cause,
            DeleteInstanceProfileError::ServiceAccount(ref cause) => cause,
            DeleteInstanceProfileError::Validation(ref cause) => cause,
            DeleteInstanceProfileError::Credentials(ref err) => err.description(),
            DeleteInstanceProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteInstanceProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteNetworkProfile
#[derive(Debug, PartialEq)]
pub enum DeleteNetworkProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteNetworkProfileError {
    pub fn from_body(body: &str) -> DeleteNetworkProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        DeleteNetworkProfileError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DeleteNetworkProfileError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteNetworkProfileError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        DeleteNetworkProfileError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteNetworkProfileError::Validation(error_message.to_string())
                    }
                    _ => DeleteNetworkProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteNetworkProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteNetworkProfileError {
    fn from(err: serde_json::error::Error) -> DeleteNetworkProfileError {
        DeleteNetworkProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteNetworkProfileError {
    fn from(err: CredentialsError) -> DeleteNetworkProfileError {
        DeleteNetworkProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteNetworkProfileError {
    fn from(err: HttpDispatchError) -> DeleteNetworkProfileError {
        DeleteNetworkProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteNetworkProfileError {
    fn from(err: io::Error) -> DeleteNetworkProfileError {
        DeleteNetworkProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteNetworkProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteNetworkProfileError {
    fn description(&self) -> &str {
        match *self {
            DeleteNetworkProfileError::Argument(ref cause) => cause,
            DeleteNetworkProfileError::LimitExceeded(ref cause) => cause,
            DeleteNetworkProfileError::NotFound(ref cause) => cause,
            DeleteNetworkProfileError::ServiceAccount(ref cause) => cause,
            DeleteNetworkProfileError::Validation(ref cause) => cause,
            DeleteNetworkProfileError::Credentials(ref err) => err.description(),
            DeleteNetworkProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteNetworkProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteProject
#[derive(Debug, PartialEq)]
pub enum DeleteProjectError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteProjectError {
    pub fn from_body(body: &str) -> DeleteProjectError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        DeleteProjectError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DeleteProjectError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteProjectError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        DeleteProjectError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteProjectError::Validation(error_message.to_string())
                    }
                    _ => DeleteProjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteProjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteProjectError {
    fn from(err: serde_json::error::Error) -> DeleteProjectError {
        DeleteProjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteProjectError {
    fn from(err: CredentialsError) -> DeleteProjectError {
        DeleteProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteProjectError {
    fn from(err: HttpDispatchError) -> DeleteProjectError {
        DeleteProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteProjectError {
    fn from(err: io::Error) -> DeleteProjectError {
        DeleteProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteProjectError {
    fn description(&self) -> &str {
        match *self {
            DeleteProjectError::Argument(ref cause) => cause,
            DeleteProjectError::LimitExceeded(ref cause) => cause,
            DeleteProjectError::NotFound(ref cause) => cause,
            DeleteProjectError::ServiceAccount(ref cause) => cause,
            DeleteProjectError::Validation(ref cause) => cause,
            DeleteProjectError::Credentials(ref err) => err.description(),
            DeleteProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteProjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRemoteAccessSession
#[derive(Debug, PartialEq)]
pub enum DeleteRemoteAccessSessionError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteRemoteAccessSessionError {
    pub fn from_body(body: &str) -> DeleteRemoteAccessSessionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        DeleteRemoteAccessSessionError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DeleteRemoteAccessSessionError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteRemoteAccessSessionError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        DeleteRemoteAccessSessionError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteRemoteAccessSessionError::Validation(error_message.to_string())
                    }
                    _ => DeleteRemoteAccessSessionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRemoteAccessSessionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRemoteAccessSessionError {
    fn from(err: serde_json::error::Error) -> DeleteRemoteAccessSessionError {
        DeleteRemoteAccessSessionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRemoteAccessSessionError {
    fn from(err: CredentialsError) -> DeleteRemoteAccessSessionError {
        DeleteRemoteAccessSessionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRemoteAccessSessionError {
    fn from(err: HttpDispatchError) -> DeleteRemoteAccessSessionError {
        DeleteRemoteAccessSessionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteRemoteAccessSessionError {
    fn from(err: io::Error) -> DeleteRemoteAccessSessionError {
        DeleteRemoteAccessSessionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteRemoteAccessSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRemoteAccessSessionError {
    fn description(&self) -> &str {
        match *self {
            DeleteRemoteAccessSessionError::Argument(ref cause) => cause,
            DeleteRemoteAccessSessionError::LimitExceeded(ref cause) => cause,
            DeleteRemoteAccessSessionError::NotFound(ref cause) => cause,
            DeleteRemoteAccessSessionError::ServiceAccount(ref cause) => cause,
            DeleteRemoteAccessSessionError::Validation(ref cause) => cause,
            DeleteRemoteAccessSessionError::Credentials(ref err) => err.description(),
            DeleteRemoteAccessSessionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteRemoteAccessSessionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRun
#[derive(Debug, PartialEq)]
pub enum DeleteRunError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteRunError {
    pub fn from_body(body: &str) -> DeleteRunError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => DeleteRunError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        DeleteRunError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => DeleteRunError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        DeleteRunError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => DeleteRunError::Validation(error_message.to_string()),
                    _ => DeleteRunError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRunError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRunError {
    fn from(err: serde_json::error::Error) -> DeleteRunError {
        DeleteRunError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRunError {
    fn from(err: CredentialsError) -> DeleteRunError {
        DeleteRunError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRunError {
    fn from(err: HttpDispatchError) -> DeleteRunError {
        DeleteRunError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteRunError {
    fn from(err: io::Error) -> DeleteRunError {
        DeleteRunError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRunError {
    fn description(&self) -> &str {
        match *self {
            DeleteRunError::Argument(ref cause) => cause,
            DeleteRunError::LimitExceeded(ref cause) => cause,
            DeleteRunError::NotFound(ref cause) => cause,
            DeleteRunError::ServiceAccount(ref cause) => cause,
            DeleteRunError::Validation(ref cause) => cause,
            DeleteRunError::Credentials(ref err) => err.description(),
            DeleteRunError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteRunError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUpload
#[derive(Debug, PartialEq)]
pub enum DeleteUploadError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteUploadError {
    pub fn from_body(body: &str) -> DeleteUploadError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => DeleteUploadError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        DeleteUploadError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => DeleteUploadError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        DeleteUploadError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteUploadError::Validation(error_message.to_string())
                    }
                    _ => DeleteUploadError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteUploadError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteUploadError {
    fn from(err: serde_json::error::Error) -> DeleteUploadError {
        DeleteUploadError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteUploadError {
    fn from(err: CredentialsError) -> DeleteUploadError {
        DeleteUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteUploadError {
    fn from(err: HttpDispatchError) -> DeleteUploadError {
        DeleteUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteUploadError {
    fn from(err: io::Error) -> DeleteUploadError {
        DeleteUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUploadError {
    fn description(&self) -> &str {
        match *self {
            DeleteUploadError::Argument(ref cause) => cause,
            DeleteUploadError::LimitExceeded(ref cause) => cause,
            DeleteUploadError::NotFound(ref cause) => cause,
            DeleteUploadError::ServiceAccount(ref cause) => cause,
            DeleteUploadError::Validation(ref cause) => cause,
            DeleteUploadError::Credentials(ref err) => err.description(),
            DeleteUploadError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVPCEConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteVPCEConfigurationError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>There was an error with the update request, or you do not have sufficient permissions to update this VPC endpoint configuration.</p>
    InvalidOperation(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteVPCEConfigurationError {
    pub fn from_body(body: &str) -> DeleteVPCEConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        DeleteVPCEConfigurationError::Argument(String::from(error_message))
                    }
                    "InvalidOperationException" => {
                        DeleteVPCEConfigurationError::InvalidOperation(String::from(error_message))
                    }
                    "NotFoundException" => {
                        DeleteVPCEConfigurationError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        DeleteVPCEConfigurationError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteVPCEConfigurationError::Validation(error_message.to_string())
                    }
                    _ => DeleteVPCEConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteVPCEConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteVPCEConfigurationError {
    fn from(err: serde_json::error::Error) -> DeleteVPCEConfigurationError {
        DeleteVPCEConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteVPCEConfigurationError {
    fn from(err: CredentialsError) -> DeleteVPCEConfigurationError {
        DeleteVPCEConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteVPCEConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteVPCEConfigurationError {
        DeleteVPCEConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteVPCEConfigurationError {
    fn from(err: io::Error) -> DeleteVPCEConfigurationError {
        DeleteVPCEConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteVPCEConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVPCEConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteVPCEConfigurationError::Argument(ref cause) => cause,
            DeleteVPCEConfigurationError::InvalidOperation(ref cause) => cause,
            DeleteVPCEConfigurationError::NotFound(ref cause) => cause,
            DeleteVPCEConfigurationError::ServiceAccount(ref cause) => cause,
            DeleteVPCEConfigurationError::Validation(ref cause) => cause,
            DeleteVPCEConfigurationError::Credentials(ref err) => err.description(),
            DeleteVPCEConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteVPCEConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAccountSettings
#[derive(Debug, PartialEq)]
pub enum GetAccountSettingsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetAccountSettingsError {
    pub fn from_body(body: &str) -> GetAccountSettingsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        GetAccountSettingsError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetAccountSettingsError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetAccountSettingsError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        GetAccountSettingsError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetAccountSettingsError::Validation(error_message.to_string())
                    }
                    _ => GetAccountSettingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAccountSettingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAccountSettingsError {
    fn from(err: serde_json::error::Error) -> GetAccountSettingsError {
        GetAccountSettingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAccountSettingsError {
    fn from(err: CredentialsError) -> GetAccountSettingsError {
        GetAccountSettingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAccountSettingsError {
    fn from(err: HttpDispatchError) -> GetAccountSettingsError {
        GetAccountSettingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAccountSettingsError {
    fn from(err: io::Error) -> GetAccountSettingsError {
        GetAccountSettingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAccountSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAccountSettingsError {
    fn description(&self) -> &str {
        match *self {
            GetAccountSettingsError::Argument(ref cause) => cause,
            GetAccountSettingsError::LimitExceeded(ref cause) => cause,
            GetAccountSettingsError::NotFound(ref cause) => cause,
            GetAccountSettingsError::ServiceAccount(ref cause) => cause,
            GetAccountSettingsError::Validation(ref cause) => cause,
            GetAccountSettingsError::Credentials(ref err) => err.description(),
            GetAccountSettingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetAccountSettingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDevice
#[derive(Debug, PartialEq)]
pub enum GetDeviceError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDeviceError {
    pub fn from_body(body: &str) -> GetDeviceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => GetDeviceError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        GetDeviceError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => GetDeviceError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        GetDeviceError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => GetDeviceError::Validation(error_message.to_string()),
                    _ => GetDeviceError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDeviceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDeviceError {
    fn from(err: serde_json::error::Error) -> GetDeviceError {
        GetDeviceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDeviceError {
    fn from(err: CredentialsError) -> GetDeviceError {
        GetDeviceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDeviceError {
    fn from(err: HttpDispatchError) -> GetDeviceError {
        GetDeviceError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDeviceError {
    fn from(err: io::Error) -> GetDeviceError {
        GetDeviceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeviceError {
    fn description(&self) -> &str {
        match *self {
            GetDeviceError::Argument(ref cause) => cause,
            GetDeviceError::LimitExceeded(ref cause) => cause,
            GetDeviceError::NotFound(ref cause) => cause,
            GetDeviceError::ServiceAccount(ref cause) => cause,
            GetDeviceError::Validation(ref cause) => cause,
            GetDeviceError::Credentials(ref err) => err.description(),
            GetDeviceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDeviceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDeviceInstance
#[derive(Debug, PartialEq)]
pub enum GetDeviceInstanceError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDeviceInstanceError {
    pub fn from_body(body: &str) -> GetDeviceInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        GetDeviceInstanceError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetDeviceInstanceError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetDeviceInstanceError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        GetDeviceInstanceError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDeviceInstanceError::Validation(error_message.to_string())
                    }
                    _ => GetDeviceInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDeviceInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDeviceInstanceError {
    fn from(err: serde_json::error::Error) -> GetDeviceInstanceError {
        GetDeviceInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDeviceInstanceError {
    fn from(err: CredentialsError) -> GetDeviceInstanceError {
        GetDeviceInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDeviceInstanceError {
    fn from(err: HttpDispatchError) -> GetDeviceInstanceError {
        GetDeviceInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDeviceInstanceError {
    fn from(err: io::Error) -> GetDeviceInstanceError {
        GetDeviceInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDeviceInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeviceInstanceError {
    fn description(&self) -> &str {
        match *self {
            GetDeviceInstanceError::Argument(ref cause) => cause,
            GetDeviceInstanceError::LimitExceeded(ref cause) => cause,
            GetDeviceInstanceError::NotFound(ref cause) => cause,
            GetDeviceInstanceError::ServiceAccount(ref cause) => cause,
            GetDeviceInstanceError::Validation(ref cause) => cause,
            GetDeviceInstanceError::Credentials(ref err) => err.description(),
            GetDeviceInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDeviceInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDevicePool
#[derive(Debug, PartialEq)]
pub enum GetDevicePoolError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDevicePoolError {
    pub fn from_body(body: &str) -> GetDevicePoolError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        GetDevicePoolError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetDevicePoolError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetDevicePoolError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        GetDevicePoolError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDevicePoolError::Validation(error_message.to_string())
                    }
                    _ => GetDevicePoolError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDevicePoolError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDevicePoolError {
    fn from(err: serde_json::error::Error) -> GetDevicePoolError {
        GetDevicePoolError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDevicePoolError {
    fn from(err: CredentialsError) -> GetDevicePoolError {
        GetDevicePoolError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDevicePoolError {
    fn from(err: HttpDispatchError) -> GetDevicePoolError {
        GetDevicePoolError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDevicePoolError {
    fn from(err: io::Error) -> GetDevicePoolError {
        GetDevicePoolError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDevicePoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDevicePoolError {
    fn description(&self) -> &str {
        match *self {
            GetDevicePoolError::Argument(ref cause) => cause,
            GetDevicePoolError::LimitExceeded(ref cause) => cause,
            GetDevicePoolError::NotFound(ref cause) => cause,
            GetDevicePoolError::ServiceAccount(ref cause) => cause,
            GetDevicePoolError::Validation(ref cause) => cause,
            GetDevicePoolError::Credentials(ref err) => err.description(),
            GetDevicePoolError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetDevicePoolError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDevicePoolCompatibility
#[derive(Debug, PartialEq)]
pub enum GetDevicePoolCompatibilityError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDevicePoolCompatibilityError {
    pub fn from_body(body: &str) -> GetDevicePoolCompatibilityError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        GetDevicePoolCompatibilityError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetDevicePoolCompatibilityError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetDevicePoolCompatibilityError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        GetDevicePoolCompatibilityError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDevicePoolCompatibilityError::Validation(error_message.to_string())
                    }
                    _ => GetDevicePoolCompatibilityError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDevicePoolCompatibilityError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDevicePoolCompatibilityError {
    fn from(err: serde_json::error::Error) -> GetDevicePoolCompatibilityError {
        GetDevicePoolCompatibilityError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDevicePoolCompatibilityError {
    fn from(err: CredentialsError) -> GetDevicePoolCompatibilityError {
        GetDevicePoolCompatibilityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDevicePoolCompatibilityError {
    fn from(err: HttpDispatchError) -> GetDevicePoolCompatibilityError {
        GetDevicePoolCompatibilityError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDevicePoolCompatibilityError {
    fn from(err: io::Error) -> GetDevicePoolCompatibilityError {
        GetDevicePoolCompatibilityError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDevicePoolCompatibilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDevicePoolCompatibilityError {
    fn description(&self) -> &str {
        match *self {
            GetDevicePoolCompatibilityError::Argument(ref cause) => cause,
            GetDevicePoolCompatibilityError::LimitExceeded(ref cause) => cause,
            GetDevicePoolCompatibilityError::NotFound(ref cause) => cause,
            GetDevicePoolCompatibilityError::ServiceAccount(ref cause) => cause,
            GetDevicePoolCompatibilityError::Validation(ref cause) => cause,
            GetDevicePoolCompatibilityError::Credentials(ref err) => err.description(),
            GetDevicePoolCompatibilityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDevicePoolCompatibilityError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstanceProfile
#[derive(Debug, PartialEq)]
pub enum GetInstanceProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetInstanceProfileError {
    pub fn from_body(body: &str) -> GetInstanceProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        GetInstanceProfileError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetInstanceProfileError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetInstanceProfileError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        GetInstanceProfileError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetInstanceProfileError::Validation(error_message.to_string())
                    }
                    _ => GetInstanceProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetInstanceProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetInstanceProfileError {
    fn from(err: serde_json::error::Error) -> GetInstanceProfileError {
        GetInstanceProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetInstanceProfileError {
    fn from(err: CredentialsError) -> GetInstanceProfileError {
        GetInstanceProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetInstanceProfileError {
    fn from(err: HttpDispatchError) -> GetInstanceProfileError {
        GetInstanceProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetInstanceProfileError {
    fn from(err: io::Error) -> GetInstanceProfileError {
        GetInstanceProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetInstanceProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstanceProfileError {
    fn description(&self) -> &str {
        match *self {
            GetInstanceProfileError::Argument(ref cause) => cause,
            GetInstanceProfileError::LimitExceeded(ref cause) => cause,
            GetInstanceProfileError::NotFound(ref cause) => cause,
            GetInstanceProfileError::ServiceAccount(ref cause) => cause,
            GetInstanceProfileError::Validation(ref cause) => cause,
            GetInstanceProfileError::Credentials(ref err) => err.description(),
            GetInstanceProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetInstanceProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJob
#[derive(Debug, PartialEq)]
pub enum GetJobError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetJobError {
    pub fn from_body(body: &str) -> GetJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => GetJobError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        GetJobError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => GetJobError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        GetJobError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => GetJobError::Validation(error_message.to_string()),
                    _ => GetJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetJobError {
    fn from(err: serde_json::error::Error) -> GetJobError {
        GetJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetJobError {
    fn from(err: CredentialsError) -> GetJobError {
        GetJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetJobError {
    fn from(err: HttpDispatchError) -> GetJobError {
        GetJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetJobError {
    fn from(err: io::Error) -> GetJobError {
        GetJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobError {
    fn description(&self) -> &str {
        match *self {
            GetJobError::Argument(ref cause) => cause,
            GetJobError::LimitExceeded(ref cause) => cause,
            GetJobError::NotFound(ref cause) => cause,
            GetJobError::ServiceAccount(ref cause) => cause,
            GetJobError::Validation(ref cause) => cause,
            GetJobError::Credentials(ref err) => err.description(),
            GetJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetNetworkProfile
#[derive(Debug, PartialEq)]
pub enum GetNetworkProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetNetworkProfileError {
    pub fn from_body(body: &str) -> GetNetworkProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        GetNetworkProfileError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetNetworkProfileError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetNetworkProfileError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        GetNetworkProfileError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetNetworkProfileError::Validation(error_message.to_string())
                    }
                    _ => GetNetworkProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetNetworkProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetNetworkProfileError {
    fn from(err: serde_json::error::Error) -> GetNetworkProfileError {
        GetNetworkProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetNetworkProfileError {
    fn from(err: CredentialsError) -> GetNetworkProfileError {
        GetNetworkProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetNetworkProfileError {
    fn from(err: HttpDispatchError) -> GetNetworkProfileError {
        GetNetworkProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetNetworkProfileError {
    fn from(err: io::Error) -> GetNetworkProfileError {
        GetNetworkProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetNetworkProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetNetworkProfileError {
    fn description(&self) -> &str {
        match *self {
            GetNetworkProfileError::Argument(ref cause) => cause,
            GetNetworkProfileError::LimitExceeded(ref cause) => cause,
            GetNetworkProfileError::NotFound(ref cause) => cause,
            GetNetworkProfileError::ServiceAccount(ref cause) => cause,
            GetNetworkProfileError::Validation(ref cause) => cause,
            GetNetworkProfileError::Credentials(ref err) => err.description(),
            GetNetworkProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetNetworkProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetOfferingStatus
#[derive(Debug, PartialEq)]
pub enum GetOfferingStatusError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>Exception gets thrown when a user is not eligible to perform the specified transaction.</p>
    NotEligible(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetOfferingStatusError {
    pub fn from_body(body: &str) -> GetOfferingStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        GetOfferingStatusError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetOfferingStatusError::LimitExceeded(String::from(error_message))
                    }
                    "NotEligibleException" => {
                        GetOfferingStatusError::NotEligible(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetOfferingStatusError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        GetOfferingStatusError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetOfferingStatusError::Validation(error_message.to_string())
                    }
                    _ => GetOfferingStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetOfferingStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetOfferingStatusError {
    fn from(err: serde_json::error::Error) -> GetOfferingStatusError {
        GetOfferingStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetOfferingStatusError {
    fn from(err: CredentialsError) -> GetOfferingStatusError {
        GetOfferingStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetOfferingStatusError {
    fn from(err: HttpDispatchError) -> GetOfferingStatusError {
        GetOfferingStatusError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetOfferingStatusError {
    fn from(err: io::Error) -> GetOfferingStatusError {
        GetOfferingStatusError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetOfferingStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetOfferingStatusError {
    fn description(&self) -> &str {
        match *self {
            GetOfferingStatusError::Argument(ref cause) => cause,
            GetOfferingStatusError::LimitExceeded(ref cause) => cause,
            GetOfferingStatusError::NotEligible(ref cause) => cause,
            GetOfferingStatusError::NotFound(ref cause) => cause,
            GetOfferingStatusError::ServiceAccount(ref cause) => cause,
            GetOfferingStatusError::Validation(ref cause) => cause,
            GetOfferingStatusError::Credentials(ref err) => err.description(),
            GetOfferingStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetOfferingStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetProject
#[derive(Debug, PartialEq)]
pub enum GetProjectError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetProjectError {
    pub fn from_body(body: &str) -> GetProjectError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => GetProjectError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        GetProjectError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => GetProjectError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        GetProjectError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => GetProjectError::Validation(error_message.to_string()),
                    _ => GetProjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetProjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetProjectError {
    fn from(err: serde_json::error::Error) -> GetProjectError {
        GetProjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetProjectError {
    fn from(err: CredentialsError) -> GetProjectError {
        GetProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetProjectError {
    fn from(err: HttpDispatchError) -> GetProjectError {
        GetProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetProjectError {
    fn from(err: io::Error) -> GetProjectError {
        GetProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetProjectError {
    fn description(&self) -> &str {
        match *self {
            GetProjectError::Argument(ref cause) => cause,
            GetProjectError::LimitExceeded(ref cause) => cause,
            GetProjectError::NotFound(ref cause) => cause,
            GetProjectError::ServiceAccount(ref cause) => cause,
            GetProjectError::Validation(ref cause) => cause,
            GetProjectError::Credentials(ref err) => err.description(),
            GetProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetProjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRemoteAccessSession
#[derive(Debug, PartialEq)]
pub enum GetRemoteAccessSessionError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetRemoteAccessSessionError {
    pub fn from_body(body: &str) -> GetRemoteAccessSessionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        GetRemoteAccessSessionError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetRemoteAccessSessionError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetRemoteAccessSessionError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        GetRemoteAccessSessionError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetRemoteAccessSessionError::Validation(error_message.to_string())
                    }
                    _ => GetRemoteAccessSessionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRemoteAccessSessionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRemoteAccessSessionError {
    fn from(err: serde_json::error::Error) -> GetRemoteAccessSessionError {
        GetRemoteAccessSessionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRemoteAccessSessionError {
    fn from(err: CredentialsError) -> GetRemoteAccessSessionError {
        GetRemoteAccessSessionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRemoteAccessSessionError {
    fn from(err: HttpDispatchError) -> GetRemoteAccessSessionError {
        GetRemoteAccessSessionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetRemoteAccessSessionError {
    fn from(err: io::Error) -> GetRemoteAccessSessionError {
        GetRemoteAccessSessionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetRemoteAccessSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRemoteAccessSessionError {
    fn description(&self) -> &str {
        match *self {
            GetRemoteAccessSessionError::Argument(ref cause) => cause,
            GetRemoteAccessSessionError::LimitExceeded(ref cause) => cause,
            GetRemoteAccessSessionError::NotFound(ref cause) => cause,
            GetRemoteAccessSessionError::ServiceAccount(ref cause) => cause,
            GetRemoteAccessSessionError::Validation(ref cause) => cause,
            GetRemoteAccessSessionError::Credentials(ref err) => err.description(),
            GetRemoteAccessSessionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetRemoteAccessSessionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRun
#[derive(Debug, PartialEq)]
pub enum GetRunError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetRunError {
    pub fn from_body(body: &str) -> GetRunError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => GetRunError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        GetRunError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => GetRunError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        GetRunError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => GetRunError::Validation(error_message.to_string()),
                    _ => GetRunError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRunError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRunError {
    fn from(err: serde_json::error::Error) -> GetRunError {
        GetRunError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRunError {
    fn from(err: CredentialsError) -> GetRunError {
        GetRunError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRunError {
    fn from(err: HttpDispatchError) -> GetRunError {
        GetRunError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetRunError {
    fn from(err: io::Error) -> GetRunError {
        GetRunError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRunError {
    fn description(&self) -> &str {
        match *self {
            GetRunError::Argument(ref cause) => cause,
            GetRunError::LimitExceeded(ref cause) => cause,
            GetRunError::NotFound(ref cause) => cause,
            GetRunError::ServiceAccount(ref cause) => cause,
            GetRunError::Validation(ref cause) => cause,
            GetRunError::Credentials(ref err) => err.description(),
            GetRunError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetRunError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSuite
#[derive(Debug, PartialEq)]
pub enum GetSuiteError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetSuiteError {
    pub fn from_body(body: &str) -> GetSuiteError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => GetSuiteError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        GetSuiteError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => GetSuiteError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        GetSuiteError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => GetSuiteError::Validation(error_message.to_string()),
                    _ => GetSuiteError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetSuiteError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetSuiteError {
    fn from(err: serde_json::error::Error) -> GetSuiteError {
        GetSuiteError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetSuiteError {
    fn from(err: CredentialsError) -> GetSuiteError {
        GetSuiteError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetSuiteError {
    fn from(err: HttpDispatchError) -> GetSuiteError {
        GetSuiteError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetSuiteError {
    fn from(err: io::Error) -> GetSuiteError {
        GetSuiteError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetSuiteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSuiteError {
    fn description(&self) -> &str {
        match *self {
            GetSuiteError::Argument(ref cause) => cause,
            GetSuiteError::LimitExceeded(ref cause) => cause,
            GetSuiteError::NotFound(ref cause) => cause,
            GetSuiteError::ServiceAccount(ref cause) => cause,
            GetSuiteError::Validation(ref cause) => cause,
            GetSuiteError::Credentials(ref err) => err.description(),
            GetSuiteError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetSuiteError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTest
#[derive(Debug, PartialEq)]
pub enum GetTestError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetTestError {
    pub fn from_body(body: &str) -> GetTestError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => GetTestError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        GetTestError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => GetTestError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        GetTestError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => GetTestError::Validation(error_message.to_string()),
                    _ => GetTestError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTestError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTestError {
    fn from(err: serde_json::error::Error) -> GetTestError {
        GetTestError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTestError {
    fn from(err: CredentialsError) -> GetTestError {
        GetTestError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTestError {
    fn from(err: HttpDispatchError) -> GetTestError {
        GetTestError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTestError {
    fn from(err: io::Error) -> GetTestError {
        GetTestError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTestError {
    fn description(&self) -> &str {
        match *self {
            GetTestError::Argument(ref cause) => cause,
            GetTestError::LimitExceeded(ref cause) => cause,
            GetTestError::NotFound(ref cause) => cause,
            GetTestError::ServiceAccount(ref cause) => cause,
            GetTestError::Validation(ref cause) => cause,
            GetTestError::Credentials(ref err) => err.description(),
            GetTestError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTestError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUpload
#[derive(Debug, PartialEq)]
pub enum GetUploadError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetUploadError {
    pub fn from_body(body: &str) -> GetUploadError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => GetUploadError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        GetUploadError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => GetUploadError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        GetUploadError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => GetUploadError::Validation(error_message.to_string()),
                    _ => GetUploadError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetUploadError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetUploadError {
    fn from(err: serde_json::error::Error) -> GetUploadError {
        GetUploadError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetUploadError {
    fn from(err: CredentialsError) -> GetUploadError {
        GetUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetUploadError {
    fn from(err: HttpDispatchError) -> GetUploadError {
        GetUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetUploadError {
    fn from(err: io::Error) -> GetUploadError {
        GetUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUploadError {
    fn description(&self) -> &str {
        match *self {
            GetUploadError::Argument(ref cause) => cause,
            GetUploadError::LimitExceeded(ref cause) => cause,
            GetUploadError::NotFound(ref cause) => cause,
            GetUploadError::ServiceAccount(ref cause) => cause,
            GetUploadError::Validation(ref cause) => cause,
            GetUploadError::Credentials(ref err) => err.description(),
            GetUploadError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetVPCEConfiguration
#[derive(Debug, PartialEq)]
pub enum GetVPCEConfigurationError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetVPCEConfigurationError {
    pub fn from_body(body: &str) -> GetVPCEConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        GetVPCEConfigurationError::Argument(String::from(error_message))
                    }
                    "NotFoundException" => {
                        GetVPCEConfigurationError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        GetVPCEConfigurationError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetVPCEConfigurationError::Validation(error_message.to_string())
                    }
                    _ => GetVPCEConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetVPCEConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetVPCEConfigurationError {
    fn from(err: serde_json::error::Error) -> GetVPCEConfigurationError {
        GetVPCEConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetVPCEConfigurationError {
    fn from(err: CredentialsError) -> GetVPCEConfigurationError {
        GetVPCEConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetVPCEConfigurationError {
    fn from(err: HttpDispatchError) -> GetVPCEConfigurationError {
        GetVPCEConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetVPCEConfigurationError {
    fn from(err: io::Error) -> GetVPCEConfigurationError {
        GetVPCEConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetVPCEConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVPCEConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetVPCEConfigurationError::Argument(ref cause) => cause,
            GetVPCEConfigurationError::NotFound(ref cause) => cause,
            GetVPCEConfigurationError::ServiceAccount(ref cause) => cause,
            GetVPCEConfigurationError::Validation(ref cause) => cause,
            GetVPCEConfigurationError::Credentials(ref err) => err.description(),
            GetVPCEConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetVPCEConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by InstallToRemoteAccessSession
#[derive(Debug, PartialEq)]
pub enum InstallToRemoteAccessSessionError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl InstallToRemoteAccessSessionError {
    pub fn from_body(body: &str) -> InstallToRemoteAccessSessionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        InstallToRemoteAccessSessionError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => InstallToRemoteAccessSessionError::LimitExceeded(
                        String::from(error_message),
                    ),
                    "NotFoundException" => {
                        InstallToRemoteAccessSessionError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => InstallToRemoteAccessSessionError::ServiceAccount(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        InstallToRemoteAccessSessionError::Validation(error_message.to_string())
                    }
                    _ => InstallToRemoteAccessSessionError::Unknown(String::from(body)),
                }
            }
            Err(_) => InstallToRemoteAccessSessionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for InstallToRemoteAccessSessionError {
    fn from(err: serde_json::error::Error) -> InstallToRemoteAccessSessionError {
        InstallToRemoteAccessSessionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for InstallToRemoteAccessSessionError {
    fn from(err: CredentialsError) -> InstallToRemoteAccessSessionError {
        InstallToRemoteAccessSessionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InstallToRemoteAccessSessionError {
    fn from(err: HttpDispatchError) -> InstallToRemoteAccessSessionError {
        InstallToRemoteAccessSessionError::HttpDispatch(err)
    }
}
impl From<io::Error> for InstallToRemoteAccessSessionError {
    fn from(err: io::Error) -> InstallToRemoteAccessSessionError {
        InstallToRemoteAccessSessionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for InstallToRemoteAccessSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InstallToRemoteAccessSessionError {
    fn description(&self) -> &str {
        match *self {
            InstallToRemoteAccessSessionError::Argument(ref cause) => cause,
            InstallToRemoteAccessSessionError::LimitExceeded(ref cause) => cause,
            InstallToRemoteAccessSessionError::NotFound(ref cause) => cause,
            InstallToRemoteAccessSessionError::ServiceAccount(ref cause) => cause,
            InstallToRemoteAccessSessionError::Validation(ref cause) => cause,
            InstallToRemoteAccessSessionError::Credentials(ref err) => err.description(),
            InstallToRemoteAccessSessionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            InstallToRemoteAccessSessionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListArtifacts
#[derive(Debug, PartialEq)]
pub enum ListArtifactsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListArtifactsError {
    pub fn from_body(body: &str) -> ListArtifactsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        ListArtifactsError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListArtifactsError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListArtifactsError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        ListArtifactsError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListArtifactsError::Validation(error_message.to_string())
                    }
                    _ => ListArtifactsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListArtifactsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListArtifactsError {
    fn from(err: serde_json::error::Error) -> ListArtifactsError {
        ListArtifactsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListArtifactsError {
    fn from(err: CredentialsError) -> ListArtifactsError {
        ListArtifactsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListArtifactsError {
    fn from(err: HttpDispatchError) -> ListArtifactsError {
        ListArtifactsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListArtifactsError {
    fn from(err: io::Error) -> ListArtifactsError {
        ListArtifactsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListArtifactsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListArtifactsError {
    fn description(&self) -> &str {
        match *self {
            ListArtifactsError::Argument(ref cause) => cause,
            ListArtifactsError::LimitExceeded(ref cause) => cause,
            ListArtifactsError::NotFound(ref cause) => cause,
            ListArtifactsError::ServiceAccount(ref cause) => cause,
            ListArtifactsError::Validation(ref cause) => cause,
            ListArtifactsError::Credentials(ref err) => err.description(),
            ListArtifactsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListArtifactsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDeviceInstances
#[derive(Debug, PartialEq)]
pub enum ListDeviceInstancesError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListDeviceInstancesError {
    pub fn from_body(body: &str) -> ListDeviceInstancesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        ListDeviceInstancesError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListDeviceInstancesError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListDeviceInstancesError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        ListDeviceInstancesError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListDeviceInstancesError::Validation(error_message.to_string())
                    }
                    _ => ListDeviceInstancesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDeviceInstancesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDeviceInstancesError {
    fn from(err: serde_json::error::Error) -> ListDeviceInstancesError {
        ListDeviceInstancesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDeviceInstancesError {
    fn from(err: CredentialsError) -> ListDeviceInstancesError {
        ListDeviceInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDeviceInstancesError {
    fn from(err: HttpDispatchError) -> ListDeviceInstancesError {
        ListDeviceInstancesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDeviceInstancesError {
    fn from(err: io::Error) -> ListDeviceInstancesError {
        ListDeviceInstancesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDeviceInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeviceInstancesError {
    fn description(&self) -> &str {
        match *self {
            ListDeviceInstancesError::Argument(ref cause) => cause,
            ListDeviceInstancesError::LimitExceeded(ref cause) => cause,
            ListDeviceInstancesError::NotFound(ref cause) => cause,
            ListDeviceInstancesError::ServiceAccount(ref cause) => cause,
            ListDeviceInstancesError::Validation(ref cause) => cause,
            ListDeviceInstancesError::Credentials(ref err) => err.description(),
            ListDeviceInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDeviceInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDevicePools
#[derive(Debug, PartialEq)]
pub enum ListDevicePoolsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListDevicePoolsError {
    pub fn from_body(body: &str) -> ListDevicePoolsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        ListDevicePoolsError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListDevicePoolsError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListDevicePoolsError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        ListDevicePoolsError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListDevicePoolsError::Validation(error_message.to_string())
                    }
                    _ => ListDevicePoolsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDevicePoolsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDevicePoolsError {
    fn from(err: serde_json::error::Error) -> ListDevicePoolsError {
        ListDevicePoolsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDevicePoolsError {
    fn from(err: CredentialsError) -> ListDevicePoolsError {
        ListDevicePoolsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDevicePoolsError {
    fn from(err: HttpDispatchError) -> ListDevicePoolsError {
        ListDevicePoolsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDevicePoolsError {
    fn from(err: io::Error) -> ListDevicePoolsError {
        ListDevicePoolsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDevicePoolsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDevicePoolsError {
    fn description(&self) -> &str {
        match *self {
            ListDevicePoolsError::Argument(ref cause) => cause,
            ListDevicePoolsError::LimitExceeded(ref cause) => cause,
            ListDevicePoolsError::NotFound(ref cause) => cause,
            ListDevicePoolsError::ServiceAccount(ref cause) => cause,
            ListDevicePoolsError::Validation(ref cause) => cause,
            ListDevicePoolsError::Credentials(ref err) => err.description(),
            ListDevicePoolsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDevicePoolsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDevices
#[derive(Debug, PartialEq)]
pub enum ListDevicesError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListDevicesError {
    pub fn from_body(body: &str) -> ListDevicesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => ListDevicesError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        ListDevicesError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => ListDevicesError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        ListDevicesError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListDevicesError::Validation(error_message.to_string())
                    }
                    _ => ListDevicesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDevicesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDevicesError {
    fn from(err: serde_json::error::Error) -> ListDevicesError {
        ListDevicesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDevicesError {
    fn from(err: CredentialsError) -> ListDevicesError {
        ListDevicesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDevicesError {
    fn from(err: HttpDispatchError) -> ListDevicesError {
        ListDevicesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDevicesError {
    fn from(err: io::Error) -> ListDevicesError {
        ListDevicesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDevicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDevicesError {
    fn description(&self) -> &str {
        match *self {
            ListDevicesError::Argument(ref cause) => cause,
            ListDevicesError::LimitExceeded(ref cause) => cause,
            ListDevicesError::NotFound(ref cause) => cause,
            ListDevicesError::ServiceAccount(ref cause) => cause,
            ListDevicesError::Validation(ref cause) => cause,
            ListDevicesError::Credentials(ref err) => err.description(),
            ListDevicesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDevicesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListInstanceProfiles
#[derive(Debug, PartialEq)]
pub enum ListInstanceProfilesError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListInstanceProfilesError {
    pub fn from_body(body: &str) -> ListInstanceProfilesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        ListInstanceProfilesError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListInstanceProfilesError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListInstanceProfilesError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        ListInstanceProfilesError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListInstanceProfilesError::Validation(error_message.to_string())
                    }
                    _ => ListInstanceProfilesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListInstanceProfilesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListInstanceProfilesError {
    fn from(err: serde_json::error::Error) -> ListInstanceProfilesError {
        ListInstanceProfilesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListInstanceProfilesError {
    fn from(err: CredentialsError) -> ListInstanceProfilesError {
        ListInstanceProfilesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListInstanceProfilesError {
    fn from(err: HttpDispatchError) -> ListInstanceProfilesError {
        ListInstanceProfilesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListInstanceProfilesError {
    fn from(err: io::Error) -> ListInstanceProfilesError {
        ListInstanceProfilesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListInstanceProfilesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListInstanceProfilesError {
    fn description(&self) -> &str {
        match *self {
            ListInstanceProfilesError::Argument(ref cause) => cause,
            ListInstanceProfilesError::LimitExceeded(ref cause) => cause,
            ListInstanceProfilesError::NotFound(ref cause) => cause,
            ListInstanceProfilesError::ServiceAccount(ref cause) => cause,
            ListInstanceProfilesError::Validation(ref cause) => cause,
            ListInstanceProfilesError::Credentials(ref err) => err.description(),
            ListInstanceProfilesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListInstanceProfilesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListJobsError {
    pub fn from_body(body: &str) -> ListJobsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => ListJobsError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        ListJobsError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => ListJobsError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        ListJobsError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => ListJobsError::Validation(error_message.to_string()),
                    _ => ListJobsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListJobsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListJobsError {
    fn from(err: serde_json::error::Error) -> ListJobsError {
        ListJobsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListJobsError {
    fn from(err: CredentialsError) -> ListJobsError {
        ListJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListJobsError {
    fn from(err: HttpDispatchError) -> ListJobsError {
        ListJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListJobsError {
    fn from(err: io::Error) -> ListJobsError {
        ListJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListJobsError {
    fn description(&self) -> &str {
        match *self {
            ListJobsError::Argument(ref cause) => cause,
            ListJobsError::LimitExceeded(ref cause) => cause,
            ListJobsError::NotFound(ref cause) => cause,
            ListJobsError::ServiceAccount(ref cause) => cause,
            ListJobsError::Validation(ref cause) => cause,
            ListJobsError::Credentials(ref err) => err.description(),
            ListJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListJobsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListNetworkProfiles
#[derive(Debug, PartialEq)]
pub enum ListNetworkProfilesError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListNetworkProfilesError {
    pub fn from_body(body: &str) -> ListNetworkProfilesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        ListNetworkProfilesError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListNetworkProfilesError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListNetworkProfilesError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        ListNetworkProfilesError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListNetworkProfilesError::Validation(error_message.to_string())
                    }
                    _ => ListNetworkProfilesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListNetworkProfilesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListNetworkProfilesError {
    fn from(err: serde_json::error::Error) -> ListNetworkProfilesError {
        ListNetworkProfilesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListNetworkProfilesError {
    fn from(err: CredentialsError) -> ListNetworkProfilesError {
        ListNetworkProfilesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListNetworkProfilesError {
    fn from(err: HttpDispatchError) -> ListNetworkProfilesError {
        ListNetworkProfilesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListNetworkProfilesError {
    fn from(err: io::Error) -> ListNetworkProfilesError {
        ListNetworkProfilesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListNetworkProfilesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListNetworkProfilesError {
    fn description(&self) -> &str {
        match *self {
            ListNetworkProfilesError::Argument(ref cause) => cause,
            ListNetworkProfilesError::LimitExceeded(ref cause) => cause,
            ListNetworkProfilesError::NotFound(ref cause) => cause,
            ListNetworkProfilesError::ServiceAccount(ref cause) => cause,
            ListNetworkProfilesError::Validation(ref cause) => cause,
            ListNetworkProfilesError::Credentials(ref err) => err.description(),
            ListNetworkProfilesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListNetworkProfilesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListOfferingPromotions
#[derive(Debug, PartialEq)]
pub enum ListOfferingPromotionsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>Exception gets thrown when a user is not eligible to perform the specified transaction.</p>
    NotEligible(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListOfferingPromotionsError {
    pub fn from_body(body: &str) -> ListOfferingPromotionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        ListOfferingPromotionsError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListOfferingPromotionsError::LimitExceeded(String::from(error_message))
                    }
                    "NotEligibleException" => {
                        ListOfferingPromotionsError::NotEligible(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListOfferingPromotionsError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        ListOfferingPromotionsError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListOfferingPromotionsError::Validation(error_message.to_string())
                    }
                    _ => ListOfferingPromotionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListOfferingPromotionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListOfferingPromotionsError {
    fn from(err: serde_json::error::Error) -> ListOfferingPromotionsError {
        ListOfferingPromotionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListOfferingPromotionsError {
    fn from(err: CredentialsError) -> ListOfferingPromotionsError {
        ListOfferingPromotionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListOfferingPromotionsError {
    fn from(err: HttpDispatchError) -> ListOfferingPromotionsError {
        ListOfferingPromotionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListOfferingPromotionsError {
    fn from(err: io::Error) -> ListOfferingPromotionsError {
        ListOfferingPromotionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListOfferingPromotionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOfferingPromotionsError {
    fn description(&self) -> &str {
        match *self {
            ListOfferingPromotionsError::Argument(ref cause) => cause,
            ListOfferingPromotionsError::LimitExceeded(ref cause) => cause,
            ListOfferingPromotionsError::NotEligible(ref cause) => cause,
            ListOfferingPromotionsError::NotFound(ref cause) => cause,
            ListOfferingPromotionsError::ServiceAccount(ref cause) => cause,
            ListOfferingPromotionsError::Validation(ref cause) => cause,
            ListOfferingPromotionsError::Credentials(ref err) => err.description(),
            ListOfferingPromotionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListOfferingPromotionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListOfferingTransactions
#[derive(Debug, PartialEq)]
pub enum ListOfferingTransactionsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>Exception gets thrown when a user is not eligible to perform the specified transaction.</p>
    NotEligible(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListOfferingTransactionsError {
    pub fn from_body(body: &str) -> ListOfferingTransactionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        ListOfferingTransactionsError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListOfferingTransactionsError::LimitExceeded(String::from(error_message))
                    }
                    "NotEligibleException" => {
                        ListOfferingTransactionsError::NotEligible(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListOfferingTransactionsError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        ListOfferingTransactionsError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListOfferingTransactionsError::Validation(error_message.to_string())
                    }
                    _ => ListOfferingTransactionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListOfferingTransactionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListOfferingTransactionsError {
    fn from(err: serde_json::error::Error) -> ListOfferingTransactionsError {
        ListOfferingTransactionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListOfferingTransactionsError {
    fn from(err: CredentialsError) -> ListOfferingTransactionsError {
        ListOfferingTransactionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListOfferingTransactionsError {
    fn from(err: HttpDispatchError) -> ListOfferingTransactionsError {
        ListOfferingTransactionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListOfferingTransactionsError {
    fn from(err: io::Error) -> ListOfferingTransactionsError {
        ListOfferingTransactionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListOfferingTransactionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOfferingTransactionsError {
    fn description(&self) -> &str {
        match *self {
            ListOfferingTransactionsError::Argument(ref cause) => cause,
            ListOfferingTransactionsError::LimitExceeded(ref cause) => cause,
            ListOfferingTransactionsError::NotEligible(ref cause) => cause,
            ListOfferingTransactionsError::NotFound(ref cause) => cause,
            ListOfferingTransactionsError::ServiceAccount(ref cause) => cause,
            ListOfferingTransactionsError::Validation(ref cause) => cause,
            ListOfferingTransactionsError::Credentials(ref err) => err.description(),
            ListOfferingTransactionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListOfferingTransactionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListOfferings
#[derive(Debug, PartialEq)]
pub enum ListOfferingsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>Exception gets thrown when a user is not eligible to perform the specified transaction.</p>
    NotEligible(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListOfferingsError {
    pub fn from_body(body: &str) -> ListOfferingsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        ListOfferingsError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListOfferingsError::LimitExceeded(String::from(error_message))
                    }
                    "NotEligibleException" => {
                        ListOfferingsError::NotEligible(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListOfferingsError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        ListOfferingsError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListOfferingsError::Validation(error_message.to_string())
                    }
                    _ => ListOfferingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListOfferingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListOfferingsError {
    fn from(err: serde_json::error::Error) -> ListOfferingsError {
        ListOfferingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListOfferingsError {
    fn from(err: CredentialsError) -> ListOfferingsError {
        ListOfferingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListOfferingsError {
    fn from(err: HttpDispatchError) -> ListOfferingsError {
        ListOfferingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListOfferingsError {
    fn from(err: io::Error) -> ListOfferingsError {
        ListOfferingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListOfferingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOfferingsError {
    fn description(&self) -> &str {
        match *self {
            ListOfferingsError::Argument(ref cause) => cause,
            ListOfferingsError::LimitExceeded(ref cause) => cause,
            ListOfferingsError::NotEligible(ref cause) => cause,
            ListOfferingsError::NotFound(ref cause) => cause,
            ListOfferingsError::ServiceAccount(ref cause) => cause,
            ListOfferingsError::Validation(ref cause) => cause,
            ListOfferingsError::Credentials(ref err) => err.description(),
            ListOfferingsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListOfferingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListProjects
#[derive(Debug, PartialEq)]
pub enum ListProjectsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListProjectsError {
    pub fn from_body(body: &str) -> ListProjectsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => ListProjectsError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        ListProjectsError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => ListProjectsError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        ListProjectsError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListProjectsError::Validation(error_message.to_string())
                    }
                    _ => ListProjectsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListProjectsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListProjectsError {
    fn from(err: serde_json::error::Error) -> ListProjectsError {
        ListProjectsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListProjectsError {
    fn from(err: CredentialsError) -> ListProjectsError {
        ListProjectsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListProjectsError {
    fn from(err: HttpDispatchError) -> ListProjectsError {
        ListProjectsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListProjectsError {
    fn from(err: io::Error) -> ListProjectsError {
        ListProjectsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListProjectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListProjectsError {
    fn description(&self) -> &str {
        match *self {
            ListProjectsError::Argument(ref cause) => cause,
            ListProjectsError::LimitExceeded(ref cause) => cause,
            ListProjectsError::NotFound(ref cause) => cause,
            ListProjectsError::ServiceAccount(ref cause) => cause,
            ListProjectsError::Validation(ref cause) => cause,
            ListProjectsError::Credentials(ref err) => err.description(),
            ListProjectsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListProjectsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRemoteAccessSessions
#[derive(Debug, PartialEq)]
pub enum ListRemoteAccessSessionsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListRemoteAccessSessionsError {
    pub fn from_body(body: &str) -> ListRemoteAccessSessionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        ListRemoteAccessSessionsError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListRemoteAccessSessionsError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListRemoteAccessSessionsError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        ListRemoteAccessSessionsError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListRemoteAccessSessionsError::Validation(error_message.to_string())
                    }
                    _ => ListRemoteAccessSessionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListRemoteAccessSessionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListRemoteAccessSessionsError {
    fn from(err: serde_json::error::Error) -> ListRemoteAccessSessionsError {
        ListRemoteAccessSessionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListRemoteAccessSessionsError {
    fn from(err: CredentialsError) -> ListRemoteAccessSessionsError {
        ListRemoteAccessSessionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListRemoteAccessSessionsError {
    fn from(err: HttpDispatchError) -> ListRemoteAccessSessionsError {
        ListRemoteAccessSessionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListRemoteAccessSessionsError {
    fn from(err: io::Error) -> ListRemoteAccessSessionsError {
        ListRemoteAccessSessionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListRemoteAccessSessionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRemoteAccessSessionsError {
    fn description(&self) -> &str {
        match *self {
            ListRemoteAccessSessionsError::Argument(ref cause) => cause,
            ListRemoteAccessSessionsError::LimitExceeded(ref cause) => cause,
            ListRemoteAccessSessionsError::NotFound(ref cause) => cause,
            ListRemoteAccessSessionsError::ServiceAccount(ref cause) => cause,
            ListRemoteAccessSessionsError::Validation(ref cause) => cause,
            ListRemoteAccessSessionsError::Credentials(ref err) => err.description(),
            ListRemoteAccessSessionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListRemoteAccessSessionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRuns
#[derive(Debug, PartialEq)]
pub enum ListRunsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListRunsError {
    pub fn from_body(body: &str) -> ListRunsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => ListRunsError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        ListRunsError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => ListRunsError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        ListRunsError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => ListRunsError::Validation(error_message.to_string()),
                    _ => ListRunsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListRunsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListRunsError {
    fn from(err: serde_json::error::Error) -> ListRunsError {
        ListRunsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListRunsError {
    fn from(err: CredentialsError) -> ListRunsError {
        ListRunsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListRunsError {
    fn from(err: HttpDispatchError) -> ListRunsError {
        ListRunsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListRunsError {
    fn from(err: io::Error) -> ListRunsError {
        ListRunsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRunsError {
    fn description(&self) -> &str {
        match *self {
            ListRunsError::Argument(ref cause) => cause,
            ListRunsError::LimitExceeded(ref cause) => cause,
            ListRunsError::NotFound(ref cause) => cause,
            ListRunsError::ServiceAccount(ref cause) => cause,
            ListRunsError::Validation(ref cause) => cause,
            ListRunsError::Credentials(ref err) => err.description(),
            ListRunsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListRunsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSamples
#[derive(Debug, PartialEq)]
pub enum ListSamplesError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListSamplesError {
    pub fn from_body(body: &str) -> ListSamplesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => ListSamplesError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        ListSamplesError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => ListSamplesError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        ListSamplesError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListSamplesError::Validation(error_message.to_string())
                    }
                    _ => ListSamplesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListSamplesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListSamplesError {
    fn from(err: serde_json::error::Error) -> ListSamplesError {
        ListSamplesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSamplesError {
    fn from(err: CredentialsError) -> ListSamplesError {
        ListSamplesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSamplesError {
    fn from(err: HttpDispatchError) -> ListSamplesError {
        ListSamplesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListSamplesError {
    fn from(err: io::Error) -> ListSamplesError {
        ListSamplesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListSamplesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSamplesError {
    fn description(&self) -> &str {
        match *self {
            ListSamplesError::Argument(ref cause) => cause,
            ListSamplesError::LimitExceeded(ref cause) => cause,
            ListSamplesError::NotFound(ref cause) => cause,
            ListSamplesError::ServiceAccount(ref cause) => cause,
            ListSamplesError::Validation(ref cause) => cause,
            ListSamplesError::Credentials(ref err) => err.description(),
            ListSamplesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListSamplesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSuites
#[derive(Debug, PartialEq)]
pub enum ListSuitesError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListSuitesError {
    pub fn from_body(body: &str) -> ListSuitesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => ListSuitesError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        ListSuitesError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => ListSuitesError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        ListSuitesError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => ListSuitesError::Validation(error_message.to_string()),
                    _ => ListSuitesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListSuitesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListSuitesError {
    fn from(err: serde_json::error::Error) -> ListSuitesError {
        ListSuitesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListSuitesError {
    fn from(err: CredentialsError) -> ListSuitesError {
        ListSuitesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListSuitesError {
    fn from(err: HttpDispatchError) -> ListSuitesError {
        ListSuitesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListSuitesError {
    fn from(err: io::Error) -> ListSuitesError {
        ListSuitesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListSuitesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSuitesError {
    fn description(&self) -> &str {
        match *self {
            ListSuitesError::Argument(ref cause) => cause,
            ListSuitesError::LimitExceeded(ref cause) => cause,
            ListSuitesError::NotFound(ref cause) => cause,
            ListSuitesError::ServiceAccount(ref cause) => cause,
            ListSuitesError::Validation(ref cause) => cause,
            ListSuitesError::Credentials(ref err) => err.description(),
            ListSuitesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListSuitesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTests
#[derive(Debug, PartialEq)]
pub enum ListTestsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTestsError {
    pub fn from_body(body: &str) -> ListTestsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => ListTestsError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        ListTestsError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => ListTestsError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        ListTestsError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => ListTestsError::Validation(error_message.to_string()),
                    _ => ListTestsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTestsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTestsError {
    fn from(err: serde_json::error::Error) -> ListTestsError {
        ListTestsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTestsError {
    fn from(err: CredentialsError) -> ListTestsError {
        ListTestsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTestsError {
    fn from(err: HttpDispatchError) -> ListTestsError {
        ListTestsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTestsError {
    fn from(err: io::Error) -> ListTestsError {
        ListTestsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTestsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTestsError {
    fn description(&self) -> &str {
        match *self {
            ListTestsError::Argument(ref cause) => cause,
            ListTestsError::LimitExceeded(ref cause) => cause,
            ListTestsError::NotFound(ref cause) => cause,
            ListTestsError::ServiceAccount(ref cause) => cause,
            ListTestsError::Validation(ref cause) => cause,
            ListTestsError::Credentials(ref err) => err.description(),
            ListTestsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTestsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUniqueProblems
#[derive(Debug, PartialEq)]
pub enum ListUniqueProblemsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListUniqueProblemsError {
    pub fn from_body(body: &str) -> ListUniqueProblemsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        ListUniqueProblemsError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListUniqueProblemsError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        ListUniqueProblemsError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        ListUniqueProblemsError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListUniqueProblemsError::Validation(error_message.to_string())
                    }
                    _ => ListUniqueProblemsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListUniqueProblemsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListUniqueProblemsError {
    fn from(err: serde_json::error::Error) -> ListUniqueProblemsError {
        ListUniqueProblemsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListUniqueProblemsError {
    fn from(err: CredentialsError) -> ListUniqueProblemsError {
        ListUniqueProblemsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListUniqueProblemsError {
    fn from(err: HttpDispatchError) -> ListUniqueProblemsError {
        ListUniqueProblemsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListUniqueProblemsError {
    fn from(err: io::Error) -> ListUniqueProblemsError {
        ListUniqueProblemsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListUniqueProblemsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUniqueProblemsError {
    fn description(&self) -> &str {
        match *self {
            ListUniqueProblemsError::Argument(ref cause) => cause,
            ListUniqueProblemsError::LimitExceeded(ref cause) => cause,
            ListUniqueProblemsError::NotFound(ref cause) => cause,
            ListUniqueProblemsError::ServiceAccount(ref cause) => cause,
            ListUniqueProblemsError::Validation(ref cause) => cause,
            ListUniqueProblemsError::Credentials(ref err) => err.description(),
            ListUniqueProblemsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListUniqueProblemsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUploads
#[derive(Debug, PartialEq)]
pub enum ListUploadsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListUploadsError {
    pub fn from_body(body: &str) -> ListUploadsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => ListUploadsError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        ListUploadsError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => ListUploadsError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        ListUploadsError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListUploadsError::Validation(error_message.to_string())
                    }
                    _ => ListUploadsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListUploadsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListUploadsError {
    fn from(err: serde_json::error::Error) -> ListUploadsError {
        ListUploadsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListUploadsError {
    fn from(err: CredentialsError) -> ListUploadsError {
        ListUploadsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListUploadsError {
    fn from(err: HttpDispatchError) -> ListUploadsError {
        ListUploadsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListUploadsError {
    fn from(err: io::Error) -> ListUploadsError {
        ListUploadsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListUploadsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUploadsError {
    fn description(&self) -> &str {
        match *self {
            ListUploadsError::Argument(ref cause) => cause,
            ListUploadsError::LimitExceeded(ref cause) => cause,
            ListUploadsError::NotFound(ref cause) => cause,
            ListUploadsError::ServiceAccount(ref cause) => cause,
            ListUploadsError::Validation(ref cause) => cause,
            ListUploadsError::Credentials(ref err) => err.description(),
            ListUploadsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListUploadsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListVPCEConfigurations
#[derive(Debug, PartialEq)]
pub enum ListVPCEConfigurationsError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListVPCEConfigurationsError {
    pub fn from_body(body: &str) -> ListVPCEConfigurationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        ListVPCEConfigurationsError::Argument(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        ListVPCEConfigurationsError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListVPCEConfigurationsError::Validation(error_message.to_string())
                    }
                    _ => ListVPCEConfigurationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListVPCEConfigurationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListVPCEConfigurationsError {
    fn from(err: serde_json::error::Error) -> ListVPCEConfigurationsError {
        ListVPCEConfigurationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListVPCEConfigurationsError {
    fn from(err: CredentialsError) -> ListVPCEConfigurationsError {
        ListVPCEConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListVPCEConfigurationsError {
    fn from(err: HttpDispatchError) -> ListVPCEConfigurationsError {
        ListVPCEConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListVPCEConfigurationsError {
    fn from(err: io::Error) -> ListVPCEConfigurationsError {
        ListVPCEConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListVPCEConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListVPCEConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            ListVPCEConfigurationsError::Argument(ref cause) => cause,
            ListVPCEConfigurationsError::ServiceAccount(ref cause) => cause,
            ListVPCEConfigurationsError::Validation(ref cause) => cause,
            ListVPCEConfigurationsError::Credentials(ref err) => err.description(),
            ListVPCEConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListVPCEConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PurchaseOffering
#[derive(Debug, PartialEq)]
pub enum PurchaseOfferingError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>Exception gets thrown when a user is not eligible to perform the specified transaction.</p>
    NotEligible(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PurchaseOfferingError {
    pub fn from_body(body: &str) -> PurchaseOfferingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        PurchaseOfferingError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PurchaseOfferingError::LimitExceeded(String::from(error_message))
                    }
                    "NotEligibleException" => {
                        PurchaseOfferingError::NotEligible(String::from(error_message))
                    }
                    "NotFoundException" => {
                        PurchaseOfferingError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        PurchaseOfferingError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        PurchaseOfferingError::Validation(error_message.to_string())
                    }
                    _ => PurchaseOfferingError::Unknown(String::from(body)),
                }
            }
            Err(_) => PurchaseOfferingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PurchaseOfferingError {
    fn from(err: serde_json::error::Error) -> PurchaseOfferingError {
        PurchaseOfferingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PurchaseOfferingError {
    fn from(err: CredentialsError) -> PurchaseOfferingError {
        PurchaseOfferingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PurchaseOfferingError {
    fn from(err: HttpDispatchError) -> PurchaseOfferingError {
        PurchaseOfferingError::HttpDispatch(err)
    }
}
impl From<io::Error> for PurchaseOfferingError {
    fn from(err: io::Error) -> PurchaseOfferingError {
        PurchaseOfferingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PurchaseOfferingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PurchaseOfferingError {
    fn description(&self) -> &str {
        match *self {
            PurchaseOfferingError::Argument(ref cause) => cause,
            PurchaseOfferingError::LimitExceeded(ref cause) => cause,
            PurchaseOfferingError::NotEligible(ref cause) => cause,
            PurchaseOfferingError::NotFound(ref cause) => cause,
            PurchaseOfferingError::ServiceAccount(ref cause) => cause,
            PurchaseOfferingError::Validation(ref cause) => cause,
            PurchaseOfferingError::Credentials(ref err) => err.description(),
            PurchaseOfferingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PurchaseOfferingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RenewOffering
#[derive(Debug, PartialEq)]
pub enum RenewOfferingError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>Exception gets thrown when a user is not eligible to perform the specified transaction.</p>
    NotEligible(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RenewOfferingError {
    pub fn from_body(body: &str) -> RenewOfferingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        RenewOfferingError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        RenewOfferingError::LimitExceeded(String::from(error_message))
                    }
                    "NotEligibleException" => {
                        RenewOfferingError::NotEligible(String::from(error_message))
                    }
                    "NotFoundException" => {
                        RenewOfferingError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        RenewOfferingError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        RenewOfferingError::Validation(error_message.to_string())
                    }
                    _ => RenewOfferingError::Unknown(String::from(body)),
                }
            }
            Err(_) => RenewOfferingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RenewOfferingError {
    fn from(err: serde_json::error::Error) -> RenewOfferingError {
        RenewOfferingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RenewOfferingError {
    fn from(err: CredentialsError) -> RenewOfferingError {
        RenewOfferingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RenewOfferingError {
    fn from(err: HttpDispatchError) -> RenewOfferingError {
        RenewOfferingError::HttpDispatch(err)
    }
}
impl From<io::Error> for RenewOfferingError {
    fn from(err: io::Error) -> RenewOfferingError {
        RenewOfferingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RenewOfferingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RenewOfferingError {
    fn description(&self) -> &str {
        match *self {
            RenewOfferingError::Argument(ref cause) => cause,
            RenewOfferingError::LimitExceeded(ref cause) => cause,
            RenewOfferingError::NotEligible(ref cause) => cause,
            RenewOfferingError::NotFound(ref cause) => cause,
            RenewOfferingError::ServiceAccount(ref cause) => cause,
            RenewOfferingError::Validation(ref cause) => cause,
            RenewOfferingError::Credentials(ref err) => err.description(),
            RenewOfferingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RenewOfferingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ScheduleRun
#[derive(Debug, PartialEq)]
pub enum ScheduleRunError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>An entity with the same name already exists.</p>
    Idempotency(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ScheduleRunError {
    pub fn from_body(body: &str) -> ScheduleRunError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => ScheduleRunError::Argument(String::from(error_message)),
                    "IdempotencyException" => {
                        ScheduleRunError::Idempotency(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ScheduleRunError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => ScheduleRunError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        ScheduleRunError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        ScheduleRunError::Validation(error_message.to_string())
                    }
                    _ => ScheduleRunError::Unknown(String::from(body)),
                }
            }
            Err(_) => ScheduleRunError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ScheduleRunError {
    fn from(err: serde_json::error::Error) -> ScheduleRunError {
        ScheduleRunError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ScheduleRunError {
    fn from(err: CredentialsError) -> ScheduleRunError {
        ScheduleRunError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ScheduleRunError {
    fn from(err: HttpDispatchError) -> ScheduleRunError {
        ScheduleRunError::HttpDispatch(err)
    }
}
impl From<io::Error> for ScheduleRunError {
    fn from(err: io::Error) -> ScheduleRunError {
        ScheduleRunError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ScheduleRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ScheduleRunError {
    fn description(&self) -> &str {
        match *self {
            ScheduleRunError::Argument(ref cause) => cause,
            ScheduleRunError::Idempotency(ref cause) => cause,
            ScheduleRunError::LimitExceeded(ref cause) => cause,
            ScheduleRunError::NotFound(ref cause) => cause,
            ScheduleRunError::ServiceAccount(ref cause) => cause,
            ScheduleRunError::Validation(ref cause) => cause,
            ScheduleRunError::Credentials(ref err) => err.description(),
            ScheduleRunError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ScheduleRunError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopJob
#[derive(Debug, PartialEq)]
pub enum StopJobError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StopJobError {
    pub fn from_body(body: &str) -> StopJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => StopJobError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        StopJobError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => StopJobError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        StopJobError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => StopJobError::Validation(error_message.to_string()),
                    _ => StopJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopJobError {
    fn from(err: serde_json::error::Error) -> StopJobError {
        StopJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopJobError {
    fn from(err: CredentialsError) -> StopJobError {
        StopJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopJobError {
    fn from(err: HttpDispatchError) -> StopJobError {
        StopJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopJobError {
    fn from(err: io::Error) -> StopJobError {
        StopJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopJobError {
    fn description(&self) -> &str {
        match *self {
            StopJobError::Argument(ref cause) => cause,
            StopJobError::LimitExceeded(ref cause) => cause,
            StopJobError::NotFound(ref cause) => cause,
            StopJobError::ServiceAccount(ref cause) => cause,
            StopJobError::Validation(ref cause) => cause,
            StopJobError::Credentials(ref err) => err.description(),
            StopJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopRemoteAccessSession
#[derive(Debug, PartialEq)]
pub enum StopRemoteAccessSessionError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StopRemoteAccessSessionError {
    pub fn from_body(body: &str) -> StopRemoteAccessSessionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        StopRemoteAccessSessionError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        StopRemoteAccessSessionError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        StopRemoteAccessSessionError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        StopRemoteAccessSessionError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopRemoteAccessSessionError::Validation(error_message.to_string())
                    }
                    _ => StopRemoteAccessSessionError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopRemoteAccessSessionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopRemoteAccessSessionError {
    fn from(err: serde_json::error::Error) -> StopRemoteAccessSessionError {
        StopRemoteAccessSessionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopRemoteAccessSessionError {
    fn from(err: CredentialsError) -> StopRemoteAccessSessionError {
        StopRemoteAccessSessionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopRemoteAccessSessionError {
    fn from(err: HttpDispatchError) -> StopRemoteAccessSessionError {
        StopRemoteAccessSessionError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopRemoteAccessSessionError {
    fn from(err: io::Error) -> StopRemoteAccessSessionError {
        StopRemoteAccessSessionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopRemoteAccessSessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopRemoteAccessSessionError {
    fn description(&self) -> &str {
        match *self {
            StopRemoteAccessSessionError::Argument(ref cause) => cause,
            StopRemoteAccessSessionError::LimitExceeded(ref cause) => cause,
            StopRemoteAccessSessionError::NotFound(ref cause) => cause,
            StopRemoteAccessSessionError::ServiceAccount(ref cause) => cause,
            StopRemoteAccessSessionError::Validation(ref cause) => cause,
            StopRemoteAccessSessionError::Credentials(ref err) => err.description(),
            StopRemoteAccessSessionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopRemoteAccessSessionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopRun
#[derive(Debug, PartialEq)]
pub enum StopRunError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StopRunError {
    pub fn from_body(body: &str) -> StopRunError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => StopRunError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        StopRunError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => StopRunError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        StopRunError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => StopRunError::Validation(error_message.to_string()),
                    _ => StopRunError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopRunError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopRunError {
    fn from(err: serde_json::error::Error) -> StopRunError {
        StopRunError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopRunError {
    fn from(err: CredentialsError) -> StopRunError {
        StopRunError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopRunError {
    fn from(err: HttpDispatchError) -> StopRunError {
        StopRunError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopRunError {
    fn from(err: io::Error) -> StopRunError {
        StopRunError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopRunError {
    fn description(&self) -> &str {
        match *self {
            StopRunError::Argument(ref cause) => cause,
            StopRunError::LimitExceeded(ref cause) => cause,
            StopRunError::NotFound(ref cause) => cause,
            StopRunError::ServiceAccount(ref cause) => cause,
            StopRunError::Validation(ref cause) => cause,
            StopRunError::Credentials(ref err) => err.description(),
            StopRunError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopRunError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDeviceInstance
#[derive(Debug, PartialEq)]
pub enum UpdateDeviceInstanceError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateDeviceInstanceError {
    pub fn from_body(body: &str) -> UpdateDeviceInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        UpdateDeviceInstanceError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateDeviceInstanceError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateDeviceInstanceError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        UpdateDeviceInstanceError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDeviceInstanceError::Validation(error_message.to_string())
                    }
                    _ => UpdateDeviceInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDeviceInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDeviceInstanceError {
    fn from(err: serde_json::error::Error) -> UpdateDeviceInstanceError {
        UpdateDeviceInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDeviceInstanceError {
    fn from(err: CredentialsError) -> UpdateDeviceInstanceError {
        UpdateDeviceInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDeviceInstanceError {
    fn from(err: HttpDispatchError) -> UpdateDeviceInstanceError {
        UpdateDeviceInstanceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDeviceInstanceError {
    fn from(err: io::Error) -> UpdateDeviceInstanceError {
        UpdateDeviceInstanceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDeviceInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDeviceInstanceError {
    fn description(&self) -> &str {
        match *self {
            UpdateDeviceInstanceError::Argument(ref cause) => cause,
            UpdateDeviceInstanceError::LimitExceeded(ref cause) => cause,
            UpdateDeviceInstanceError::NotFound(ref cause) => cause,
            UpdateDeviceInstanceError::ServiceAccount(ref cause) => cause,
            UpdateDeviceInstanceError::Validation(ref cause) => cause,
            UpdateDeviceInstanceError::Credentials(ref err) => err.description(),
            UpdateDeviceInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDeviceInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDevicePool
#[derive(Debug, PartialEq)]
pub enum UpdateDevicePoolError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateDevicePoolError {
    pub fn from_body(body: &str) -> UpdateDevicePoolError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        UpdateDevicePoolError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateDevicePoolError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateDevicePoolError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        UpdateDevicePoolError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDevicePoolError::Validation(error_message.to_string())
                    }
                    _ => UpdateDevicePoolError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDevicePoolError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDevicePoolError {
    fn from(err: serde_json::error::Error) -> UpdateDevicePoolError {
        UpdateDevicePoolError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDevicePoolError {
    fn from(err: CredentialsError) -> UpdateDevicePoolError {
        UpdateDevicePoolError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDevicePoolError {
    fn from(err: HttpDispatchError) -> UpdateDevicePoolError {
        UpdateDevicePoolError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDevicePoolError {
    fn from(err: io::Error) -> UpdateDevicePoolError {
        UpdateDevicePoolError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDevicePoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDevicePoolError {
    fn description(&self) -> &str {
        match *self {
            UpdateDevicePoolError::Argument(ref cause) => cause,
            UpdateDevicePoolError::LimitExceeded(ref cause) => cause,
            UpdateDevicePoolError::NotFound(ref cause) => cause,
            UpdateDevicePoolError::ServiceAccount(ref cause) => cause,
            UpdateDevicePoolError::Validation(ref cause) => cause,
            UpdateDevicePoolError::Credentials(ref err) => err.description(),
            UpdateDevicePoolError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateDevicePoolError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateInstanceProfile
#[derive(Debug, PartialEq)]
pub enum UpdateInstanceProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateInstanceProfileError {
    pub fn from_body(body: &str) -> UpdateInstanceProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        UpdateInstanceProfileError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateInstanceProfileError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateInstanceProfileError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        UpdateInstanceProfileError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateInstanceProfileError::Validation(error_message.to_string())
                    }
                    _ => UpdateInstanceProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateInstanceProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateInstanceProfileError {
    fn from(err: serde_json::error::Error) -> UpdateInstanceProfileError {
        UpdateInstanceProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateInstanceProfileError {
    fn from(err: CredentialsError) -> UpdateInstanceProfileError {
        UpdateInstanceProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateInstanceProfileError {
    fn from(err: HttpDispatchError) -> UpdateInstanceProfileError {
        UpdateInstanceProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateInstanceProfileError {
    fn from(err: io::Error) -> UpdateInstanceProfileError {
        UpdateInstanceProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateInstanceProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateInstanceProfileError {
    fn description(&self) -> &str {
        match *self {
            UpdateInstanceProfileError::Argument(ref cause) => cause,
            UpdateInstanceProfileError::LimitExceeded(ref cause) => cause,
            UpdateInstanceProfileError::NotFound(ref cause) => cause,
            UpdateInstanceProfileError::ServiceAccount(ref cause) => cause,
            UpdateInstanceProfileError::Validation(ref cause) => cause,
            UpdateInstanceProfileError::Credentials(ref err) => err.description(),
            UpdateInstanceProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateInstanceProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateNetworkProfile
#[derive(Debug, PartialEq)]
pub enum UpdateNetworkProfileError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateNetworkProfileError {
    pub fn from_body(body: &str) -> UpdateNetworkProfileError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        UpdateNetworkProfileError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateNetworkProfileError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateNetworkProfileError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        UpdateNetworkProfileError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateNetworkProfileError::Validation(error_message.to_string())
                    }
                    _ => UpdateNetworkProfileError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateNetworkProfileError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateNetworkProfileError {
    fn from(err: serde_json::error::Error) -> UpdateNetworkProfileError {
        UpdateNetworkProfileError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateNetworkProfileError {
    fn from(err: CredentialsError) -> UpdateNetworkProfileError {
        UpdateNetworkProfileError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateNetworkProfileError {
    fn from(err: HttpDispatchError) -> UpdateNetworkProfileError {
        UpdateNetworkProfileError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateNetworkProfileError {
    fn from(err: io::Error) -> UpdateNetworkProfileError {
        UpdateNetworkProfileError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateNetworkProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateNetworkProfileError {
    fn description(&self) -> &str {
        match *self {
            UpdateNetworkProfileError::Argument(ref cause) => cause,
            UpdateNetworkProfileError::LimitExceeded(ref cause) => cause,
            UpdateNetworkProfileError::NotFound(ref cause) => cause,
            UpdateNetworkProfileError::ServiceAccount(ref cause) => cause,
            UpdateNetworkProfileError::Validation(ref cause) => cause,
            UpdateNetworkProfileError::Credentials(ref err) => err.description(),
            UpdateNetworkProfileError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateNetworkProfileError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateProject
#[derive(Debug, PartialEq)]
pub enum UpdateProjectError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateProjectError {
    pub fn from_body(body: &str) -> UpdateProjectError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        UpdateProjectError::Argument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateProjectError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateProjectError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        UpdateProjectError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateProjectError::Validation(error_message.to_string())
                    }
                    _ => UpdateProjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateProjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateProjectError {
    fn from(err: serde_json::error::Error) -> UpdateProjectError {
        UpdateProjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateProjectError {
    fn from(err: CredentialsError) -> UpdateProjectError {
        UpdateProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateProjectError {
    fn from(err: HttpDispatchError) -> UpdateProjectError {
        UpdateProjectError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateProjectError {
    fn from(err: io::Error) -> UpdateProjectError {
        UpdateProjectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateProjectError {
    fn description(&self) -> &str {
        match *self {
            UpdateProjectError::Argument(ref cause) => cause,
            UpdateProjectError::LimitExceeded(ref cause) => cause,
            UpdateProjectError::NotFound(ref cause) => cause,
            UpdateProjectError::ServiceAccount(ref cause) => cause,
            UpdateProjectError::Validation(ref cause) => cause,
            UpdateProjectError::Credentials(ref err) => err.description(),
            UpdateProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateProjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUpload
#[derive(Debug, PartialEq)]
pub enum UpdateUploadError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>A limit was exceeded.</p>
    LimitExceeded(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateUploadError {
    pub fn from_body(body: &str) -> UpdateUploadError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => UpdateUploadError::Argument(String::from(error_message)),
                    "LimitExceededException" => {
                        UpdateUploadError::LimitExceeded(String::from(error_message))
                    }
                    "NotFoundException" => UpdateUploadError::NotFound(String::from(error_message)),
                    "ServiceAccountException" => {
                        UpdateUploadError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateUploadError::Validation(error_message.to_string())
                    }
                    _ => UpdateUploadError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateUploadError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateUploadError {
    fn from(err: serde_json::error::Error) -> UpdateUploadError {
        UpdateUploadError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateUploadError {
    fn from(err: CredentialsError) -> UpdateUploadError {
        UpdateUploadError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateUploadError {
    fn from(err: HttpDispatchError) -> UpdateUploadError {
        UpdateUploadError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateUploadError {
    fn from(err: io::Error) -> UpdateUploadError {
        UpdateUploadError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateUploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUploadError {
    fn description(&self) -> &str {
        match *self {
            UpdateUploadError::Argument(ref cause) => cause,
            UpdateUploadError::LimitExceeded(ref cause) => cause,
            UpdateUploadError::NotFound(ref cause) => cause,
            UpdateUploadError::ServiceAccount(ref cause) => cause,
            UpdateUploadError::Validation(ref cause) => cause,
            UpdateUploadError::Credentials(ref err) => err.description(),
            UpdateUploadError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateUploadError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateVPCEConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateVPCEConfigurationError {
    /// <p>An invalid argument was specified.</p>
    Argument(String),
    /// <p>There was an error with the update request, or you do not have sufficient permissions to update this VPC endpoint configuration.</p>
    InvalidOperation(String),
    /// <p>The specified entity was not found.</p>
    NotFound(String),
    /// <p>There was a problem with the service account.</p>
    ServiceAccount(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateVPCEConfigurationError {
    pub fn from_body(body: &str) -> UpdateVPCEConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ArgumentException" => {
                        UpdateVPCEConfigurationError::Argument(String::from(error_message))
                    }
                    "InvalidOperationException" => {
                        UpdateVPCEConfigurationError::InvalidOperation(String::from(error_message))
                    }
                    "NotFoundException" => {
                        UpdateVPCEConfigurationError::NotFound(String::from(error_message))
                    }
                    "ServiceAccountException" => {
                        UpdateVPCEConfigurationError::ServiceAccount(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateVPCEConfigurationError::Validation(error_message.to_string())
                    }
                    _ => UpdateVPCEConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateVPCEConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateVPCEConfigurationError {
    fn from(err: serde_json::error::Error) -> UpdateVPCEConfigurationError {
        UpdateVPCEConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateVPCEConfigurationError {
    fn from(err: CredentialsError) -> UpdateVPCEConfigurationError {
        UpdateVPCEConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateVPCEConfigurationError {
    fn from(err: HttpDispatchError) -> UpdateVPCEConfigurationError {
        UpdateVPCEConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateVPCEConfigurationError {
    fn from(err: io::Error) -> UpdateVPCEConfigurationError {
        UpdateVPCEConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateVPCEConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateVPCEConfigurationError {
    fn description(&self) -> &str {
        match *self {
            UpdateVPCEConfigurationError::Argument(ref cause) => cause,
            UpdateVPCEConfigurationError::InvalidOperation(ref cause) => cause,
            UpdateVPCEConfigurationError::NotFound(ref cause) => cause,
            UpdateVPCEConfigurationError::ServiceAccount(ref cause) => cause,
            UpdateVPCEConfigurationError::Validation(ref cause) => cause,
            UpdateVPCEConfigurationError::Credentials(ref err) => err.description(),
            UpdateVPCEConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateVPCEConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Device Farm API. AWS Device Farm clients implement this trait.
pub trait DeviceFarm {
    /// <p>Creates a device pool.</p>
    fn create_device_pool(
        &self,
        input: CreateDevicePoolRequest,
    ) -> RusotoFuture<CreateDevicePoolResult, CreateDevicePoolError>;

    /// <p>Creates a profile that can be applied to one or more private fleet device instances.</p>
    fn create_instance_profile(
        &self,
        input: CreateInstanceProfileRequest,
    ) -> RusotoFuture<CreateInstanceProfileResult, CreateInstanceProfileError>;

    /// <p>Creates a network profile.</p>
    fn create_network_profile(
        &self,
        input: CreateNetworkProfileRequest,
    ) -> RusotoFuture<CreateNetworkProfileResult, CreateNetworkProfileError>;

    /// <p>Creates a new project.</p>
    fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> RusotoFuture<CreateProjectResult, CreateProjectError>;

    /// <p>Specifies and starts a remote access session.</p>
    fn create_remote_access_session(
        &self,
        input: CreateRemoteAccessSessionRequest,
    ) -> RusotoFuture<CreateRemoteAccessSessionResult, CreateRemoteAccessSessionError>;

    /// <p>Uploads an app or test scripts.</p>
    fn create_upload(
        &self,
        input: CreateUploadRequest,
    ) -> RusotoFuture<CreateUploadResult, CreateUploadError>;

    /// <p>Creates a configuration record in Device Farm for your Amazon Virtual Private Cloud (VPC) endpoint.</p>
    fn create_vpce_configuration(
        &self,
        input: CreateVPCEConfigurationRequest,
    ) -> RusotoFuture<CreateVPCEConfigurationResult, CreateVPCEConfigurationError>;

    /// <p>Deletes a device pool given the pool ARN. Does not allow deletion of curated pools owned by the system.</p>
    fn delete_device_pool(
        &self,
        input: DeleteDevicePoolRequest,
    ) -> RusotoFuture<DeleteDevicePoolResult, DeleteDevicePoolError>;

    /// <p>Deletes a profile that can be applied to one or more private device instances.</p>
    fn delete_instance_profile(
        &self,
        input: DeleteInstanceProfileRequest,
    ) -> RusotoFuture<DeleteInstanceProfileResult, DeleteInstanceProfileError>;

    /// <p>Deletes a network profile.</p>
    fn delete_network_profile(
        &self,
        input: DeleteNetworkProfileRequest,
    ) -> RusotoFuture<DeleteNetworkProfileResult, DeleteNetworkProfileError>;

    /// <p>Deletes an AWS Device Farm project, given the project ARN.</p> <p> <b>Note</b> Deleting this resource does not stop an in-progress run.</p>
    fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> RusotoFuture<DeleteProjectResult, DeleteProjectError>;

    /// <p>Deletes a completed remote access session and its results.</p>
    fn delete_remote_access_session(
        &self,
        input: DeleteRemoteAccessSessionRequest,
    ) -> RusotoFuture<DeleteRemoteAccessSessionResult, DeleteRemoteAccessSessionError>;

    /// <p>Deletes the run, given the run ARN.</p> <p> <b>Note</b> Deleting this resource does not stop an in-progress run.</p>
    fn delete_run(&self, input: DeleteRunRequest) -> RusotoFuture<DeleteRunResult, DeleteRunError>;

    /// <p>Deletes an upload given the upload ARN.</p>
    fn delete_upload(
        &self,
        input: DeleteUploadRequest,
    ) -> RusotoFuture<DeleteUploadResult, DeleteUploadError>;

    /// <p>Deletes a configuration for your Amazon Virtual Private Cloud (VPC) endpoint.</p>
    fn delete_vpce_configuration(
        &self,
        input: DeleteVPCEConfigurationRequest,
    ) -> RusotoFuture<DeleteVPCEConfigurationResult, DeleteVPCEConfigurationError>;

    /// <p>Returns the number of unmetered iOS and/or unmetered Android devices that have been purchased by the account.</p>
    fn get_account_settings(
        &self,
    ) -> RusotoFuture<GetAccountSettingsResult, GetAccountSettingsError>;

    /// <p>Gets information about a unique device type.</p>
    fn get_device(&self, input: GetDeviceRequest) -> RusotoFuture<GetDeviceResult, GetDeviceError>;

    /// <p>Returns information about a device instance belonging to a private device fleet.</p>
    fn get_device_instance(
        &self,
        input: GetDeviceInstanceRequest,
    ) -> RusotoFuture<GetDeviceInstanceResult, GetDeviceInstanceError>;

    /// <p>Gets information about a device pool.</p>
    fn get_device_pool(
        &self,
        input: GetDevicePoolRequest,
    ) -> RusotoFuture<GetDevicePoolResult, GetDevicePoolError>;

    /// <p>Gets information about compatibility with a device pool.</p>
    fn get_device_pool_compatibility(
        &self,
        input: GetDevicePoolCompatibilityRequest,
    ) -> RusotoFuture<GetDevicePoolCompatibilityResult, GetDevicePoolCompatibilityError>;

    /// <p>Returns information about the specified instance profile.</p>
    fn get_instance_profile(
        &self,
        input: GetInstanceProfileRequest,
    ) -> RusotoFuture<GetInstanceProfileResult, GetInstanceProfileError>;

    /// <p>Gets information about a job.</p>
    fn get_job(&self, input: GetJobRequest) -> RusotoFuture<GetJobResult, GetJobError>;

    /// <p>Returns information about a network profile.</p>
    fn get_network_profile(
        &self,
        input: GetNetworkProfileRequest,
    ) -> RusotoFuture<GetNetworkProfileResult, GetNetworkProfileError>;

    /// <p>Gets the current status and future status of all offerings purchased by an AWS account. The response indicates how many offerings are currently available and the offerings that will be available in the next period. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. Please contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you believe that you should be able to invoke this operation.</p>
    fn get_offering_status(
        &self,
        input: GetOfferingStatusRequest,
    ) -> RusotoFuture<GetOfferingStatusResult, GetOfferingStatusError>;

    /// <p>Gets information about a project.</p>
    fn get_project(
        &self,
        input: GetProjectRequest,
    ) -> RusotoFuture<GetProjectResult, GetProjectError>;

    /// <p>Returns a link to a currently running remote access session.</p>
    fn get_remote_access_session(
        &self,
        input: GetRemoteAccessSessionRequest,
    ) -> RusotoFuture<GetRemoteAccessSessionResult, GetRemoteAccessSessionError>;

    /// <p>Gets information about a run.</p>
    fn get_run(&self, input: GetRunRequest) -> RusotoFuture<GetRunResult, GetRunError>;

    /// <p>Gets information about a suite.</p>
    fn get_suite(&self, input: GetSuiteRequest) -> RusotoFuture<GetSuiteResult, GetSuiteError>;

    /// <p>Gets information about a test.</p>
    fn get_test(&self, input: GetTestRequest) -> RusotoFuture<GetTestResult, GetTestError>;

    /// <p>Gets information about an upload.</p>
    fn get_upload(&self, input: GetUploadRequest) -> RusotoFuture<GetUploadResult, GetUploadError>;

    /// <p>Returns information about the configuration settings for your Amazon Virtual Private Cloud (VPC) endpoint.</p>
    fn get_vpce_configuration(
        &self,
        input: GetVPCEConfigurationRequest,
    ) -> RusotoFuture<GetVPCEConfigurationResult, GetVPCEConfigurationError>;

    /// <p>Installs an application to the device in a remote access session. For Android applications, the file must be in .apk format. For iOS applications, the file must be in .ipa format.</p>
    fn install_to_remote_access_session(
        &self,
        input: InstallToRemoteAccessSessionRequest,
    ) -> RusotoFuture<InstallToRemoteAccessSessionResult, InstallToRemoteAccessSessionError>;

    /// <p>Gets information about artifacts.</p>
    fn list_artifacts(
        &self,
        input: ListArtifactsRequest,
    ) -> RusotoFuture<ListArtifactsResult, ListArtifactsError>;

    /// <p>Returns information about the private device instances associated with one or more AWS accounts.</p>
    fn list_device_instances(
        &self,
        input: ListDeviceInstancesRequest,
    ) -> RusotoFuture<ListDeviceInstancesResult, ListDeviceInstancesError>;

    /// <p>Gets information about device pools.</p>
    fn list_device_pools(
        &self,
        input: ListDevicePoolsRequest,
    ) -> RusotoFuture<ListDevicePoolsResult, ListDevicePoolsError>;

    /// <p>Gets information about unique device types.</p>
    fn list_devices(
        &self,
        input: ListDevicesRequest,
    ) -> RusotoFuture<ListDevicesResult, ListDevicesError>;

    /// <p>Returns information about all the instance profiles in an AWS account.</p>
    fn list_instance_profiles(
        &self,
        input: ListInstanceProfilesRequest,
    ) -> RusotoFuture<ListInstanceProfilesResult, ListInstanceProfilesError>;

    /// <p>Gets information about jobs for a given test run.</p>
    fn list_jobs(&self, input: ListJobsRequest) -> RusotoFuture<ListJobsResult, ListJobsError>;

    /// <p>Returns the list of available network profiles.</p>
    fn list_network_profiles(
        &self,
        input: ListNetworkProfilesRequest,
    ) -> RusotoFuture<ListNetworkProfilesResult, ListNetworkProfilesError>;

    /// <p>Returns a list of offering promotions. Each offering promotion record contains the ID and description of the promotion. The API returns a <code>NotEligible</code> error if the caller is not permitted to invoke the operation. Contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you believe that you should be able to invoke this operation.</p>
    fn list_offering_promotions(
        &self,
        input: ListOfferingPromotionsRequest,
    ) -> RusotoFuture<ListOfferingPromotionsResult, ListOfferingPromotionsError>;

    /// <p>Returns a list of all historical purchases, renewals, and system renewal transactions for an AWS account. The list is paginated and ordered by a descending timestamp (most recent transactions are first). The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. Please contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you believe that you should be able to invoke this operation.</p>
    fn list_offering_transactions(
        &self,
        input: ListOfferingTransactionsRequest,
    ) -> RusotoFuture<ListOfferingTransactionsResult, ListOfferingTransactionsError>;

    /// <p>Returns a list of products or offerings that the user can manage through the API. Each offering record indicates the recurring price per unit and the frequency for that offering. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. Please contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you believe that you should be able to invoke this operation.</p>
    fn list_offerings(
        &self,
        input: ListOfferingsRequest,
    ) -> RusotoFuture<ListOfferingsResult, ListOfferingsError>;

    /// <p>Gets information about projects.</p>
    fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> RusotoFuture<ListProjectsResult, ListProjectsError>;

    /// <p>Returns a list of all currently running remote access sessions.</p>
    fn list_remote_access_sessions(
        &self,
        input: ListRemoteAccessSessionsRequest,
    ) -> RusotoFuture<ListRemoteAccessSessionsResult, ListRemoteAccessSessionsError>;

    /// <p>Gets information about runs, given an AWS Device Farm project ARN.</p>
    fn list_runs(&self, input: ListRunsRequest) -> RusotoFuture<ListRunsResult, ListRunsError>;

    /// <p>Gets information about samples, given an AWS Device Farm project ARN</p>
    fn list_samples(
        &self,
        input: ListSamplesRequest,
    ) -> RusotoFuture<ListSamplesResult, ListSamplesError>;

    /// <p>Gets information about test suites for a given job.</p>
    fn list_suites(
        &self,
        input: ListSuitesRequest,
    ) -> RusotoFuture<ListSuitesResult, ListSuitesError>;

    /// <p>Gets information about tests in a given test suite.</p>
    fn list_tests(&self, input: ListTestsRequest) -> RusotoFuture<ListTestsResult, ListTestsError>;

    /// <p>Gets information about unique problems.</p>
    fn list_unique_problems(
        &self,
        input: ListUniqueProblemsRequest,
    ) -> RusotoFuture<ListUniqueProblemsResult, ListUniqueProblemsError>;

    /// <p>Gets information about uploads, given an AWS Device Farm project ARN.</p>
    fn list_uploads(
        &self,
        input: ListUploadsRequest,
    ) -> RusotoFuture<ListUploadsResult, ListUploadsError>;

    /// <p>Returns information about all Amazon Virtual Private Cloud (VPC) endpoint configurations in the AWS account.</p>
    fn list_vpce_configurations(
        &self,
        input: ListVPCEConfigurationsRequest,
    ) -> RusotoFuture<ListVPCEConfigurationsResult, ListVPCEConfigurationsError>;

    /// <p>Immediately purchases offerings for an AWS account. Offerings renew with the latest total purchased quantity for an offering, unless the renewal was overridden. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. Please contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you believe that you should be able to invoke this operation.</p>
    fn purchase_offering(
        &self,
        input: PurchaseOfferingRequest,
    ) -> RusotoFuture<PurchaseOfferingResult, PurchaseOfferingError>;

    /// <p>Explicitly sets the quantity of devices to renew for an offering, starting from the <code>effectiveDate</code> of the next period. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. Please contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you believe that you should be able to invoke this operation.</p>
    fn renew_offering(
        &self,
        input: RenewOfferingRequest,
    ) -> RusotoFuture<RenewOfferingResult, RenewOfferingError>;

    /// <p>Schedules a run.</p>
    fn schedule_run(
        &self,
        input: ScheduleRunRequest,
    ) -> RusotoFuture<ScheduleRunResult, ScheduleRunError>;

    /// <p>Initiates a stop request for the current job. AWS Device Farm will immediately stop the job on the device where tests have not started executing, and you will not be billed for this device. On the device where tests have started executing, Setup Suite and Teardown Suite tests will run to completion before stopping execution on the device. You will be billed for Setup, Teardown, and any tests that were in progress or already completed.</p>
    fn stop_job(&self, input: StopJobRequest) -> RusotoFuture<StopJobResult, StopJobError>;

    /// <p>Ends a specified remote access session.</p>
    fn stop_remote_access_session(
        &self,
        input: StopRemoteAccessSessionRequest,
    ) -> RusotoFuture<StopRemoteAccessSessionResult, StopRemoteAccessSessionError>;

    /// <p>Initiates a stop request for the current test run. AWS Device Farm will immediately stop the run on devices where tests have not started executing, and you will not be billed for these devices. On devices where tests have started executing, Setup Suite and Teardown Suite tests will run to completion before stopping execution on those devices. You will be billed for Setup, Teardown, and any tests that were in progress or already completed.</p>
    fn stop_run(&self, input: StopRunRequest) -> RusotoFuture<StopRunResult, StopRunError>;

    /// <p>Updates information about an existing private device instance.</p>
    fn update_device_instance(
        &self,
        input: UpdateDeviceInstanceRequest,
    ) -> RusotoFuture<UpdateDeviceInstanceResult, UpdateDeviceInstanceError>;

    /// <p>Modifies the name, description, and rules in a device pool given the attributes and the pool ARN. Rule updates are all-or-nothing, meaning they can only be updated as a whole (or not at all).</p>
    fn update_device_pool(
        &self,
        input: UpdateDevicePoolRequest,
    ) -> RusotoFuture<UpdateDevicePoolResult, UpdateDevicePoolError>;

    /// <p>Updates information about an existing private device instance profile.</p>
    fn update_instance_profile(
        &self,
        input: UpdateInstanceProfileRequest,
    ) -> RusotoFuture<UpdateInstanceProfileResult, UpdateInstanceProfileError>;

    /// <p>Updates the network profile with specific settings.</p>
    fn update_network_profile(
        &self,
        input: UpdateNetworkProfileRequest,
    ) -> RusotoFuture<UpdateNetworkProfileResult, UpdateNetworkProfileError>;

    /// <p>Modifies the specified project name, given the project ARN and a new name.</p>
    fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> RusotoFuture<UpdateProjectResult, UpdateProjectError>;

    /// <p>Update an uploaded test specification (test spec).</p>
    fn update_upload(
        &self,
        input: UpdateUploadRequest,
    ) -> RusotoFuture<UpdateUploadResult, UpdateUploadError>;

    /// <p>Updates information about an existing Amazon Virtual Private Cloud (VPC) endpoint configuration.</p>
    fn update_vpce_configuration(
        &self,
        input: UpdateVPCEConfigurationRequest,
    ) -> RusotoFuture<UpdateVPCEConfigurationResult, UpdateVPCEConfigurationError>;
}
/// A client for the AWS Device Farm API.
pub struct DeviceFarmClient {
    client: Client,
    region: region::Region,
}

impl DeviceFarmClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DeviceFarmClient {
        DeviceFarmClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DeviceFarmClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        DeviceFarmClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl DeviceFarm for DeviceFarmClient {
    /// <p>Creates a device pool.</p>
    fn create_device_pool(
        &self,
        input: CreateDevicePoolRequest,
    ) -> RusotoFuture<CreateDevicePoolResult, CreateDevicePoolError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.CreateDevicePool");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDevicePoolResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDevicePoolError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a profile that can be applied to one or more private fleet device instances.</p>
    fn create_instance_profile(
        &self,
        input: CreateInstanceProfileRequest,
    ) -> RusotoFuture<CreateInstanceProfileResult, CreateInstanceProfileError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.CreateInstanceProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateInstanceProfileResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateInstanceProfileError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a network profile.</p>
    fn create_network_profile(
        &self,
        input: CreateNetworkProfileRequest,
    ) -> RusotoFuture<CreateNetworkProfileResult, CreateNetworkProfileError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.CreateNetworkProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateNetworkProfileResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateNetworkProfileError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a new project.</p>
    fn create_project(
        &self,
        input: CreateProjectRequest,
    ) -> RusotoFuture<CreateProjectResult, CreateProjectError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.CreateProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateProjectResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateProjectError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Specifies and starts a remote access session.</p>
    fn create_remote_access_session(
        &self,
        input: CreateRemoteAccessSessionRequest,
    ) -> RusotoFuture<CreateRemoteAccessSessionResult, CreateRemoteAccessSessionError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.CreateRemoteAccessSession",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateRemoteAccessSessionResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateRemoteAccessSessionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Uploads an app or test scripts.</p>
    fn create_upload(
        &self,
        input: CreateUploadRequest,
    ) -> RusotoFuture<CreateUploadResult, CreateUploadError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.CreateUpload");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateUploadResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateUploadError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a configuration record in Device Farm for your Amazon Virtual Private Cloud (VPC) endpoint.</p>
    fn create_vpce_configuration(
        &self,
        input: CreateVPCEConfigurationRequest,
    ) -> RusotoFuture<CreateVPCEConfigurationResult, CreateVPCEConfigurationError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.CreateVPCEConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateVPCEConfigurationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateVPCEConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a device pool given the pool ARN. Does not allow deletion of curated pools owned by the system.</p>
    fn delete_device_pool(
        &self,
        input: DeleteDevicePoolRequest,
    ) -> RusotoFuture<DeleteDevicePoolResult, DeleteDevicePoolError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.DeleteDevicePool");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteDevicePoolResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDevicePoolError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a profile that can be applied to one or more private device instances.</p>
    fn delete_instance_profile(
        &self,
        input: DeleteInstanceProfileRequest,
    ) -> RusotoFuture<DeleteInstanceProfileResult, DeleteInstanceProfileError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.DeleteInstanceProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteInstanceProfileResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteInstanceProfileError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a network profile.</p>
    fn delete_network_profile(
        &self,
        input: DeleteNetworkProfileRequest,
    ) -> RusotoFuture<DeleteNetworkProfileResult, DeleteNetworkProfileError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.DeleteNetworkProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteNetworkProfileResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteNetworkProfileError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes an AWS Device Farm project, given the project ARN.</p> <p> <b>Note</b> Deleting this resource does not stop an in-progress run.</p>
    fn delete_project(
        &self,
        input: DeleteProjectRequest,
    ) -> RusotoFuture<DeleteProjectResult, DeleteProjectError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.DeleteProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteProjectResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteProjectError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a completed remote access session and its results.</p>
    fn delete_remote_access_session(
        &self,
        input: DeleteRemoteAccessSessionRequest,
    ) -> RusotoFuture<DeleteRemoteAccessSessionResult, DeleteRemoteAccessSessionError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.DeleteRemoteAccessSession",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteRemoteAccessSessionResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRemoteAccessSessionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the run, given the run ARN.</p> <p> <b>Note</b> Deleting this resource does not stop an in-progress run.</p>
    fn delete_run(&self, input: DeleteRunRequest) -> RusotoFuture<DeleteRunResult, DeleteRunError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.DeleteRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteRunResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRunError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes an upload given the upload ARN.</p>
    fn delete_upload(
        &self,
        input: DeleteUploadRequest,
    ) -> RusotoFuture<DeleteUploadResult, DeleteUploadError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.DeleteUpload");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteUploadResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteUploadError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a configuration for your Amazon Virtual Private Cloud (VPC) endpoint.</p>
    fn delete_vpce_configuration(
        &self,
        input: DeleteVPCEConfigurationRequest,
    ) -> RusotoFuture<DeleteVPCEConfigurationResult, DeleteVPCEConfigurationError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.DeleteVPCEConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteVPCEConfigurationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteVPCEConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns the number of unmetered iOS and/or unmetered Android devices that have been purchased by the account.</p>
    fn get_account_settings(
        &self,
    ) -> RusotoFuture<GetAccountSettingsResult, GetAccountSettingsError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetAccountSettings");
        request.set_payload(Some(b"{}".to_vec()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetAccountSettingsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAccountSettingsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about a unique device type.</p>
    fn get_device(&self, input: GetDeviceRequest) -> RusotoFuture<GetDeviceResult, GetDeviceError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetDevice");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDeviceResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDeviceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns information about a device instance belonging to a private device fleet.</p>
    fn get_device_instance(
        &self,
        input: GetDeviceInstanceRequest,
    ) -> RusotoFuture<GetDeviceInstanceResult, GetDeviceInstanceError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetDeviceInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDeviceInstanceResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDeviceInstanceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about a device pool.</p>
    fn get_device_pool(
        &self,
        input: GetDevicePoolRequest,
    ) -> RusotoFuture<GetDevicePoolResult, GetDevicePoolError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetDevicePool");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDevicePoolResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDevicePoolError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about compatibility with a device pool.</p>
    fn get_device_pool_compatibility(
        &self,
        input: GetDevicePoolCompatibilityRequest,
    ) -> RusotoFuture<GetDevicePoolCompatibilityResult, GetDevicePoolCompatibilityError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.GetDevicePoolCompatibility",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDevicePoolCompatibilityResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDevicePoolCompatibilityError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns information about the specified instance profile.</p>
    fn get_instance_profile(
        &self,
        input: GetInstanceProfileRequest,
    ) -> RusotoFuture<GetInstanceProfileResult, GetInstanceProfileError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetInstanceProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetInstanceProfileResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetInstanceProfileError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about a job.</p>
    fn get_job(&self, input: GetJobRequest) -> RusotoFuture<GetJobResult, GetJobError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetJobResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetJobError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns information about a network profile.</p>
    fn get_network_profile(
        &self,
        input: GetNetworkProfileRequest,
    ) -> RusotoFuture<GetNetworkProfileResult, GetNetworkProfileError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetNetworkProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetNetworkProfileResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetNetworkProfileError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets the current status and future status of all offerings purchased by an AWS account. The response indicates how many offerings are currently available and the offerings that will be available in the next period. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. Please contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you believe that you should be able to invoke this operation.</p>
    fn get_offering_status(
        &self,
        input: GetOfferingStatusRequest,
    ) -> RusotoFuture<GetOfferingStatusResult, GetOfferingStatusError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetOfferingStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetOfferingStatusResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetOfferingStatusError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about a project.</p>
    fn get_project(
        &self,
        input: GetProjectRequest,
    ) -> RusotoFuture<GetProjectResult, GetProjectError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetProjectResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetProjectError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns a link to a currently running remote access session.</p>
    fn get_remote_access_session(
        &self,
        input: GetRemoteAccessSessionRequest,
    ) -> RusotoFuture<GetRemoteAccessSessionResult, GetRemoteAccessSessionError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetRemoteAccessSession");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRemoteAccessSessionResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRemoteAccessSessionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about a run.</p>
    fn get_run(&self, input: GetRunRequest) -> RusotoFuture<GetRunResult, GetRunError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetRunResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRunError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about a suite.</p>
    fn get_suite(&self, input: GetSuiteRequest) -> RusotoFuture<GetSuiteResult, GetSuiteError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetSuite");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetSuiteResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetSuiteError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about a test.</p>
    fn get_test(&self, input: GetTestRequest) -> RusotoFuture<GetTestResult, GetTestError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetTest");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetTestResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetTestError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about an upload.</p>
    fn get_upload(&self, input: GetUploadRequest) -> RusotoFuture<GetUploadResult, GetUploadError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetUpload");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetUploadResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetUploadError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns information about the configuration settings for your Amazon Virtual Private Cloud (VPC) endpoint.</p>
    fn get_vpce_configuration(
        &self,
        input: GetVPCEConfigurationRequest,
    ) -> RusotoFuture<GetVPCEConfigurationResult, GetVPCEConfigurationError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.GetVPCEConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetVPCEConfigurationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetVPCEConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Installs an application to the device in a remote access session. For Android applications, the file must be in .apk format. For iOS applications, the file must be in .ipa format.</p>
    fn install_to_remote_access_session(
        &self,
        input: InstallToRemoteAccessSessionRequest,
    ) -> RusotoFuture<InstallToRemoteAccessSessionResult, InstallToRemoteAccessSessionError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.InstallToRemoteAccessSession",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<InstallToRemoteAccessSessionResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(InstallToRemoteAccessSessionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about artifacts.</p>
    fn list_artifacts(
        &self,
        input: ListArtifactsRequest,
    ) -> RusotoFuture<ListArtifactsResult, ListArtifactsError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListArtifacts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListArtifactsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListArtifactsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns information about the private device instances associated with one or more AWS accounts.</p>
    fn list_device_instances(
        &self,
        input: ListDeviceInstancesRequest,
    ) -> RusotoFuture<ListDeviceInstancesResult, ListDeviceInstancesError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListDeviceInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListDeviceInstancesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListDeviceInstancesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about device pools.</p>
    fn list_device_pools(
        &self,
        input: ListDevicePoolsRequest,
    ) -> RusotoFuture<ListDevicePoolsResult, ListDevicePoolsError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListDevicePools");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListDevicePoolsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListDevicePoolsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about unique device types.</p>
    fn list_devices(
        &self,
        input: ListDevicesRequest,
    ) -> RusotoFuture<ListDevicesResult, ListDevicesError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListDevices");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListDevicesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListDevicesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns information about all the instance profiles in an AWS account.</p>
    fn list_instance_profiles(
        &self,
        input: ListInstanceProfilesRequest,
    ) -> RusotoFuture<ListInstanceProfilesResult, ListInstanceProfilesError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListInstanceProfiles");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListInstanceProfilesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListInstanceProfilesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about jobs for a given test run.</p>
    fn list_jobs(&self, input: ListJobsRequest) -> RusotoFuture<ListJobsResult, ListJobsError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListJobs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListJobsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListJobsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns the list of available network profiles.</p>
    fn list_network_profiles(
        &self,
        input: ListNetworkProfilesRequest,
    ) -> RusotoFuture<ListNetworkProfilesResult, ListNetworkProfilesError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListNetworkProfiles");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListNetworkProfilesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListNetworkProfilesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns a list of offering promotions. Each offering promotion record contains the ID and description of the promotion. The API returns a <code>NotEligible</code> error if the caller is not permitted to invoke the operation. Contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you believe that you should be able to invoke this operation.</p>
    fn list_offering_promotions(
        &self,
        input: ListOfferingPromotionsRequest,
    ) -> RusotoFuture<ListOfferingPromotionsResult, ListOfferingPromotionsError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListOfferingPromotions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListOfferingPromotionsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListOfferingPromotionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns a list of all historical purchases, renewals, and system renewal transactions for an AWS account. The list is paginated and ordered by a descending timestamp (most recent transactions are first). The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. Please contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you believe that you should be able to invoke this operation.</p>
    fn list_offering_transactions(
        &self,
        input: ListOfferingTransactionsRequest,
    ) -> RusotoFuture<ListOfferingTransactionsResult, ListOfferingTransactionsError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.ListOfferingTransactions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListOfferingTransactionsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListOfferingTransactionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns a list of products or offerings that the user can manage through the API. Each offering record indicates the recurring price per unit and the frequency for that offering. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. Please contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you believe that you should be able to invoke this operation.</p>
    fn list_offerings(
        &self,
        input: ListOfferingsRequest,
    ) -> RusotoFuture<ListOfferingsResult, ListOfferingsError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListOfferings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListOfferingsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListOfferingsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about projects.</p>
    fn list_projects(
        &self,
        input: ListProjectsRequest,
    ) -> RusotoFuture<ListProjectsResult, ListProjectsError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListProjects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListProjectsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListProjectsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns a list of all currently running remote access sessions.</p>
    fn list_remote_access_sessions(
        &self,
        input: ListRemoteAccessSessionsRequest,
    ) -> RusotoFuture<ListRemoteAccessSessionsResult, ListRemoteAccessSessionsError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.ListRemoteAccessSessions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListRemoteAccessSessionsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListRemoteAccessSessionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about runs, given an AWS Device Farm project ARN.</p>
    fn list_runs(&self, input: ListRunsRequest) -> RusotoFuture<ListRunsResult, ListRunsError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListRuns");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListRunsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListRunsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about samples, given an AWS Device Farm project ARN</p>
    fn list_samples(
        &self,
        input: ListSamplesRequest,
    ) -> RusotoFuture<ListSamplesResult, ListSamplesError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListSamples");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListSamplesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSamplesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about test suites for a given job.</p>
    fn list_suites(
        &self,
        input: ListSuitesRequest,
    ) -> RusotoFuture<ListSuitesResult, ListSuitesError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListSuites");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListSuitesResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSuitesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about tests in a given test suite.</p>
    fn list_tests(&self, input: ListTestsRequest) -> RusotoFuture<ListTestsResult, ListTestsError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListTests");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTestsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListTestsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about unique problems.</p>
    fn list_unique_problems(
        &self,
        input: ListUniqueProblemsRequest,
    ) -> RusotoFuture<ListUniqueProblemsResult, ListUniqueProblemsError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListUniqueProblems");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListUniqueProblemsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListUniqueProblemsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Gets information about uploads, given an AWS Device Farm project ARN.</p>
    fn list_uploads(
        &self,
        input: ListUploadsRequest,
    ) -> RusotoFuture<ListUploadsResult, ListUploadsError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListUploads");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListUploadsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListUploadsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns information about all Amazon Virtual Private Cloud (VPC) endpoint configurations in the AWS account.</p>
    fn list_vpce_configurations(
        &self,
        input: ListVPCEConfigurationsRequest,
    ) -> RusotoFuture<ListVPCEConfigurationsResult, ListVPCEConfigurationsError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ListVPCEConfigurations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListVPCEConfigurationsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListVPCEConfigurationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Immediately purchases offerings for an AWS account. Offerings renew with the latest total purchased quantity for an offering, unless the renewal was overridden. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. Please contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you believe that you should be able to invoke this operation.</p>
    fn purchase_offering(
        &self,
        input: PurchaseOfferingRequest,
    ) -> RusotoFuture<PurchaseOfferingResult, PurchaseOfferingError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.PurchaseOffering");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PurchaseOfferingResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PurchaseOfferingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Explicitly sets the quantity of devices to renew for an offering, starting from the <code>effectiveDate</code> of the next period. The API returns a <code>NotEligible</code> error if the user is not permitted to invoke the operation. Please contact <a href="mailto:aws-devicefarm-support@amazon.com">aws-devicefarm-support@amazon.com</a> if you believe that you should be able to invoke this operation.</p>
    fn renew_offering(
        &self,
        input: RenewOfferingRequest,
    ) -> RusotoFuture<RenewOfferingResult, RenewOfferingError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.RenewOffering");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<RenewOfferingResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RenewOfferingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Schedules a run.</p>
    fn schedule_run(
        &self,
        input: ScheduleRunRequest,
    ) -> RusotoFuture<ScheduleRunResult, ScheduleRunError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.ScheduleRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ScheduleRunResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ScheduleRunError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Initiates a stop request for the current job. AWS Device Farm will immediately stop the job on the device where tests have not started executing, and you will not be billed for this device. On the device where tests have started executing, Setup Suite and Teardown Suite tests will run to completion before stopping execution on the device. You will be billed for Setup, Teardown, and any tests that were in progress or already completed.</p>
    fn stop_job(&self, input: StopJobRequest) -> RusotoFuture<StopJobResult, StopJobError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.StopJob");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopJobResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StopJobError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Ends a specified remote access session.</p>
    fn stop_remote_access_session(
        &self,
        input: StopRemoteAccessSessionRequest,
    ) -> RusotoFuture<StopRemoteAccessSessionResult, StopRemoteAccessSessionError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.StopRemoteAccessSession",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopRemoteAccessSessionResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StopRemoteAccessSessionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Initiates a stop request for the current test run. AWS Device Farm will immediately stop the run on devices where tests have not started executing, and you will not be billed for these devices. On devices where tests have started executing, Setup Suite and Teardown Suite tests will run to completion before stopping execution on those devices. You will be billed for Setup, Teardown, and any tests that were in progress or already completed.</p>
    fn stop_run(&self, input: StopRunRequest) -> RusotoFuture<StopRunResult, StopRunError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.StopRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopRunResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StopRunError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates information about an existing private device instance.</p>
    fn update_device_instance(
        &self,
        input: UpdateDeviceInstanceRequest,
    ) -> RusotoFuture<UpdateDeviceInstanceResult, UpdateDeviceInstanceError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.UpdateDeviceInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateDeviceInstanceResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDeviceInstanceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Modifies the name, description, and rules in a device pool given the attributes and the pool ARN. Rule updates are all-or-nothing, meaning they can only be updated as a whole (or not at all).</p>
    fn update_device_pool(
        &self,
        input: UpdateDevicePoolRequest,
    ) -> RusotoFuture<UpdateDevicePoolResult, UpdateDevicePoolError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.UpdateDevicePool");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateDevicePoolResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDevicePoolError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates information about an existing private device instance profile.</p>
    fn update_instance_profile(
        &self,
        input: UpdateInstanceProfileRequest,
    ) -> RusotoFuture<UpdateInstanceProfileResult, UpdateInstanceProfileError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.UpdateInstanceProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateInstanceProfileResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateInstanceProfileError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates the network profile with specific settings.</p>
    fn update_network_profile(
        &self,
        input: UpdateNetworkProfileRequest,
    ) -> RusotoFuture<UpdateNetworkProfileResult, UpdateNetworkProfileError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.UpdateNetworkProfile");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateNetworkProfileResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateNetworkProfileError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Modifies the specified project name, given the project ARN and a new name.</p>
    fn update_project(
        &self,
        input: UpdateProjectRequest,
    ) -> RusotoFuture<UpdateProjectResult, UpdateProjectError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.UpdateProject");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateProjectResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateProjectError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Update an uploaded test specification (test spec).</p>
    fn update_upload(
        &self,
        input: UpdateUploadRequest,
    ) -> RusotoFuture<UpdateUploadResult, UpdateUploadError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "DeviceFarm_20150623.UpdateUpload");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateUploadResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateUploadError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates information about an existing Amazon Virtual Private Cloud (VPC) endpoint configuration.</p>
    fn update_vpce_configuration(
        &self,
        input: UpdateVPCEConfigurationRequest,
    ) -> RusotoFuture<UpdateVPCEConfigurationResult, UpdateVPCEConfigurationError> {
        let mut request = SignedRequest::new("POST", "devicefarm", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "DeviceFarm_20150623.UpdateVPCEConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateVPCEConfigurationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateVPCEConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
