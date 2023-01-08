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

WebUI.setText(findTestObject('Object Repository/Page_eZRX  Login page/input_Username_username (1)'), 'ace')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_eZRX  Login page/input_Password_password (1)'), 'liSTkvaEvsTwAdgpFhbvKQ==')

WebUI.click(findTestObject('Object Repository/Page_eZRX  Login page/button_Login (1)'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/a_Product (1)'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/a_Product_1 (1)'))

WebUI.click(findTestObject('Object Repository/Page_CMS APL/a_Add (1)'))

WebUI.setText(findTestObject('Object Repository/Page_CMS APL/input_Product Code_ProductName (1)'), 'du')

WebUI.click(findTestObject('Object Repository/Page_CMS APL/li_DUMMY ELECTRODES COMP(1PC)'))

WebUI.setText(findTestObject('Object Repository/Page_CMS APL/input_Product Code_ProductName (1)'), 'DUMMY ELECTRODES COMP(1PC)')

WebUI.setText(findTestObject('Object Repository/Page_CMS APL/div_testing (1)'), '<p>testing</p>')

WebUI.click(findTestObject('Object Repository/Page_CMS APL/p (1)'))

WebUI.setText(findTestObject('Object Repository/Page_CMS APL/div_testing (1)'), '<p style="">testing</p>')

WebUI.click(findTestObject('Object Repository/Page_CMS APL/p (1)'))

WebUI.setText(findTestObject('Object Repository/Page_CMS APL/div_testt'), '<p style="">testt</p>')

WebUI.click(findTestObject('Object Repository/Page_CMS APL/p (1)'))

WebUI.setText(findTestObject('Object Repository/Page_CMS APL/div_this'), '<p style="">this</p>')

WebUI.click(findTestObject('Object Repository/Page_CMS APL/button_Save (1)'))

WebUI.closeBrowser()

