import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://apq.ezrx.id/CMS/Account/Login?ReturnUrl=%2FCMS%2FHome')

WebUI.setText(findTestObject('Object Repository/Page_eZRX  Login page/input_Username_username'), 'ace')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_eZRX  Login page/input_Password_password'), 'liSTkvaEvsTwAdgpFhbvKQ==')

WebUI.click(findTestObject('Object Repository/Page_eZRX  Login page/button_Login'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/a_Product'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/a_Product_1'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/a_Add'))

WebUI.setText(findTestObject('Object Repository/Page_CMS APL/input_Product Code_ProductName'), 'du')

WebUI.click(findTestObject('Object Repository/Page_CMS APL/li_DUODERM CGF DRS 10X10CM (1X5PK) NAI'))

WebUI.setText(findTestObject('Object Repository/Page_CMS APL/input_Product Code_ProductName'), 'DUODERM CGF DRS 10X10CM (1X5PK) NAI')

WebUI.click(findTestObject('Object Repository/Page_CMS APL/p'))

WebUI.setText(findTestObject('Object Repository/Page_CMS APL/div_testing'), '<p style="">testing</p>')

WebUI.click(findTestObject('Object Repository/Page_CMS APL/p'))

WebUI.setText(findTestObject('Object Repository/Page_CMS APL/div_test'), '<p style="">test</p>')

WebUI.click(findTestObject('Object Repository/Page_CMS APL/p'))

WebUI.setText(findTestObject('Object Repository/Page_CMS APL/div_tess'), '<p style="">tess</p>')

WebUI.click(findTestObject('Object Repository/Page_CMS APL/p'))

WebUI.setText(findTestObject('Object Repository/Page_CMS APL/div_tesas'), '<p style="">tesas</p>')

WebUI.click(findTestObject('Object Repository/Page_CMS APL/button_Save'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/div_Product Code                           _0e65cb'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/button_Save'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/a_Dashboard'))

