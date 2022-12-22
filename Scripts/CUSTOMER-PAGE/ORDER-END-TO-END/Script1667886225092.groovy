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

WebUI.navigateToUrl('https://apq-fe.ezrx.id/')

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/a_MASUK'))

WebUI.setText(findTestObject('Object Repository/Page_APL Customer Portal/input_APL CUSTOMER PORTAL_username'), '116223')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_APL Customer Portal/input_APL CUSTOMER PORTAL_password'), 
    'xSx9uApFE48=')

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/i_APL CUSTOMER PORTAL_fa fa-eye'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/div_APL CUSTOMER PORTAL_row'))

WebUI.setText(findTestObject('Object Repository/Page_APL Customer Portal/input_APL CUSTOMER PORTAL_password_1'), 'Qwerty"12')

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/label_APL CUSTOMER PORTAL_control-label'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_MASUK'))

WebUI.setText(findTestObject('Object Repository/Page_APL Customer Portal/input_Bantuan_keywordSearchText'), 'danah')

WebUI.sendKeys(findTestObject('Object Repository/Page_APL Customer Portal/input_Bantuan_keywordSearchText'), Keys.chord(
        Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_Tambah Ke Keranjang'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/span_'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_Tambah Ke Keranjang'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/span_'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/i_Lihat Semua Notifikasi_la la-shopping-cart'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/i_Lihat Semua Notifikasi_la la-shopping-cart'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/a_Lihat Semua'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_UPDATE KERANJANG'))

WebUI.click(findTestObject('Object Repository/Page_APL Customer Portal/button_OK'))

WebUI.closeBrowser()

